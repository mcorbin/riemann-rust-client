use proto::proto;
use std::collections::HashMap;
use protobuf::{RepeatedField};
use chrono::{Utc};
use chrono::TimeZone;
use event;

pub fn proto_to_msg(proto_msg: &proto::Msg) -> event::Msg {
    let mut msg = event::Msg::new();
    if proto_msg.has_error() {
        msg.error = Some(proto_msg.get_error().to_owned());
    }
    if proto_msg.has_ok() {
        msg.ok = Some(proto_msg.get_ok());
    }
    if proto_msg.has_query() {
        let query = proto_msg.get_query();
        if query.has_string() {
            msg.query = Some(query.get_string().to_owned());
        }
    }
    let events = proto_msg.get_events();
    if events.len() > 0 {
        msg.events = Some(get_events(&proto_msg));
    }
    msg
}

/// Takes a proto Msg, returns these events.
///
/// # Example
///
/// let e = event::Event {
///         ...
///         };
/// let result = events_to_message(&vec![e]);
/// ```
pub fn get_events(message: &proto::Msg) -> Vec<event::Event> {
    message.get_events().to_vec().iter().map(|e| proto_to_event(e)).collect()
}

/// Creates a proto Msg containing the events received in parameter
///
/// # Example
///
/// let e = event::Event {
///         ...
///         };
/// let result = events_to_message(&vec![e]);
pub fn events_to_message(events: &Vec<event::Event>) -> proto::Msg {
    let mut msg = proto::Msg::new();
    let proto_events = events.iter().map(|e| event_to_proto(e)).collect();
    msg.set_events(RepeatedField::from_vec(proto_events));
    msg
}

/// Converts a event::Event to a proto::Event
///
/// # Example
///
/// let e = event::Event {
///         ...
///         };
/// let result = event_to_proto(&e);
pub fn event_to_proto(event: &event::Event) -> proto::Event {
    let mut e = proto::Event::new();
    if let Some(ref time) = event.time {
        let seconds = time.timestamp();
        let microseconds = time.timestamp_subsec_micros() as i64;
        let microseconds_ts = (seconds * 1_000_000) + microseconds;

        e.set_time(seconds);
        e.set_time_micros(microseconds_ts);
    }
    if let Some(ref state) = event.state {
        e.set_state(state.to_owned())
    }
    if let Some(ref service) = event.service {
        e.set_service(service.to_owned())
    }
    if let Some(ref host) = event.host {
        e.set_host(host.to_owned())
    }
    if let Some(ref description) = event.description {
        e.set_description(description.to_owned())
    }
    if let Some(ref tags) = event.tags {
        e.set_tags(RepeatedField::from_vec(tags.clone()))
    }
    if let Some(ref ttl) = event.ttl {
        e.set_ttl(*ttl)
    }
    if let Some(ref metric) = event.metric {
        match *metric {
            event::Metric::Int64(m) => e.set_metric_sint64(m),
            event::Metric::Double(m) => e.set_metric_d(m),
            event::Metric::Float(m) => e.set_metric_f(m),
        }
    }
    if let Some(ref attributes) = event.attributes {
        e.set_attributes(attributes.iter()
                         .map(|(k, v)| {
                             let mut attr = proto::Attribute::new();
                             attr.set_key(k.to_owned());
                             attr.set_value(v.to_owned());
                             attr
        }).collect())
    }
    e
}

/// Convert a proto::Event to a event::Event
///
/// # Example
///
/// let mut e = proto::Event::new();
/// let result = proto_to_event(&e);
pub fn proto_to_event(proto_event: &proto::Event) -> event::Event {
    let mut e = event::Event::new();

    // time_micros priority
    if proto_event.has_time_micros() {
        let microseconds_ts = proto_event.get_time_micros();
        let seconds = microseconds_ts/1_000_000;
        let nanoseconds = (microseconds_ts*1000) - (seconds*1_000_000_000);
        let dt = Utc.timestamp(seconds, nanoseconds as u32);
        e.time = Some(dt);
    }
    else if proto_event.has_time() {
        let seconds = proto_event.get_time();
        let dt = Utc.timestamp(seconds, 0);
        e.time = Some(dt);
    }

    if proto_event.has_state() {
        e.state = Some(proto_event.get_state().to_owned());
    }
    if proto_event.has_service() {
        e.service = Some(proto_event.get_service().to_owned());
    }
    if proto_event.has_host() {
        e.host = Some(proto_event.get_host().to_owned());
    }
    if proto_event.has_description() {
        e.description = Some(proto_event.get_description().to_owned());
    }
    if proto_event.has_ttl() {
        e.ttl = Some(proto_event.get_ttl());
    }
    // metric priority
    if proto_event.has_metric_sint64() {
        e.metric = Some(event::Metric::Int64(proto_event.get_metric_sint64()));
    }
    else if proto_event.has_metric_d() {
        e.metric = Some(event::Metric::Double(proto_event.get_metric_d()));
    }
    else if proto_event.has_metric_f() {
        e.metric = Some(event::Metric::Float(proto_event.get_metric_f()));
    }

    let tags = proto_event.get_tags();
    match tags.len() {
        0 => e.tags = None,
        _ => e.tags = Some(tags.to_vec())
    }
    let attributes = proto_event.get_attributes();
    match attributes.len() {
        0 => e.attributes = None,
        _ => e.attributes = {
            let mut attr_map = HashMap::new();
            for a in attributes {
                attr_map.insert(a.get_key().to_owned(), a.get_value().to_owned());
            }
            Some(attr_map)
        }
    }
    e
}

#[cfg(test)]
mod tests {
    use super::{event_to_proto, proto_to_event};
    use std::collections::HashMap;
    use protobuf::{RepeatedField};
    use proto::proto;
    use chrono::{Utc};
    use chrono::TimeZone;
    use event;

    #[test]
    fn proto_to_event_test() {
        let mut e = proto::Event::new();
        e.set_time(1);
        e.set_time_micros(1000001);
        e.set_state("critical".to_owned());
        e.set_service("foo".to_owned());
        e.set_host("bar".to_owned());
        e.set_description("description".to_owned());
        e.set_ttl(10.0);
        e.set_metric_sint64(10);
        e.set_metric_d(10.1);
        e.set_metric_f(10.2);
        e.set_tags(RepeatedField::from_vec(vec!["t1".to_owned(), "t2".to_owned()]));
        let mut attr = proto::Attribute::new();
        attr.set_key("k1".to_owned());
        attr.set_value("v1".to_owned());
        e.set_attributes(RepeatedField::from_vec(vec![attr]));

        let result = proto_to_event(&e);
        let mut attr = HashMap::new();
        attr.insert("k1".to_owned(), "v1".to_owned());

        assert_eq!(result.time, Some(Utc.timestamp(1, 1000)));
        assert_eq!(result.state, Some("critical".to_owned()));
        assert_eq!(result.service, Some("foo".to_owned()));
        assert_eq!(result.host, Some("bar".to_owned()));
        assert_eq!(result.description, Some("description".to_owned()));
        assert_eq!(result.tags, Some(vec!["t1".to_owned(), "t2".to_owned()]));
        assert_eq!(result.ttl, Some(10.0));
        assert_eq!(result.attributes, Some(attr));

        // how to compare enum with float easily ?
        match result.metric {
            Some(event::Metric::Int64(v)) => assert_eq!(v, 10),
            _ => panic!("error in test")
        }

        let mut e = proto::Event::new();
        e.set_metric_d(10.1);
        e.set_metric_f(10.2);
        e.set_time(10);

        let result = proto_to_event(&e);

        assert_eq!(result.time, Some(Utc.timestamp(10, 0)));
        match result.metric {
            Some(event::Metric::Double(v)) => assert_eq!(v, 10.1),
            _ => panic!("error in test")
        }

        let mut e = proto::Event::new();
        e.set_metric_f(10.2);
        e.set_time(10);

        let result = proto_to_event(&e);

        assert_eq!(result.time, Some(Utc.timestamp(10, 0)));
        match result.metric {
            Some(event::Metric::Float(v)) => assert_eq!(v, 10.2),
            _ => panic!("error in test")
        }
    }

    #[test]
    fn event_to_proto_test() {
        let mut attr = HashMap::new();
        attr.insert("foo".to_owned(), "bar".to_owned());
        let e = event::Event {
            time: Some(Utc.timestamp(1, 1000)),
            state: Some("critical".to_owned()),
            service: Some("foo".to_owned()),
            host: Some("bar".to_owned()),
            description: Some("description".to_owned()),
            tags: Some(vec!["t1".to_owned(), "t2".to_owned()]),
            ttl: Some(12.1),
            attributes: Some(attr),
            metric: Some(event::Metric::Int64(10))
        };

        let result = event_to_proto(&e);
        assert_eq!(result.get_time(), 1);
        assert_eq!(result.get_time_micros(), 1000001);
        assert_eq!(result.get_state(), "critical");
        assert_eq!(result.get_service(), "foo");
        assert_eq!(result.get_host(), "bar");
        assert_eq!(result.get_description(), "description");
        assert_eq!(result.get_ttl(), 12.1);
        assert_eq!(result.get_metric_sint64(), 10);
        assert_eq!(result.has_metric_f(), false);
        assert_eq!(result.has_metric_d(), false);
        assert_eq!(result.get_tags(), ["t1".to_owned(), "t2".to_owned()]);
        let res_attr = result.get_attributes();
        assert_eq!(res_attr.len(), 1);
        let mut attr =  proto::Attribute::new();
        attr.set_key("foo".to_owned());
        attr.set_value("bar".to_owned());
        assert_eq!(res_attr[0], attr);

        let result = event_to_proto(&event::Event::new());
        assert_eq!(result.has_time(), false);
        assert_eq!(result.has_time_micros(), false);
        assert_eq!(result.has_state(), false);
        assert_eq!(result.has_service(), false);
        assert_eq!(result.has_host(), false);
        assert_eq!(result.has_description(), false);
        assert_eq!(result.has_ttl(), false);
        assert_eq!(result.has_metric_sint64(), false);
        assert_eq!(result.has_metric_f(), false);
        assert_eq!(result.has_metric_d(), false);
        assert_eq!(result.get_tags().len(), 0);
        assert_eq!(result.get_attributes().len(), 0);

        let mut e = event::Event::new();
        e.time = Some(Utc.timestamp(0, 10000));
        let result = event_to_proto(&e);
        assert_eq!(result.get_time(), 0);
        assert_eq!(result.get_time_micros(), 10);

        let mut e = event::Event::new();
        e.time = Some(Utc.timestamp(10, 0));
        let result = event_to_proto(&e);
        assert_eq!(result.get_time(), 10);
        assert_eq!(result.get_time_micros(), 10000000);

        let mut e = event::Event::new();
        e.metric = Some(event::Metric::Double(10.1));
        let result = event_to_proto(&e);
        assert_eq!(result.get_metric_d(), 10.1);
        assert_eq!(result.has_metric_sint64(), false);
        assert_eq!(result.has_metric_f(), false);

        let mut e = event::Event::new();
        e.metric = Some(event::Metric::Int64(10));
        let result = event_to_proto(&e);
        assert_eq!(result.get_metric_sint64(), 10);
        assert_eq!(result.has_metric_d(), false);
        assert_eq!(result.has_metric_f(), false);

        let mut e = event::Event::new();
        e.metric = Some(event::Metric::Float(10.2));
        let result = event_to_proto(&e);
        assert_eq!(result.get_metric_f(), 10.2);
        assert_eq!(result.has_metric_d(), false);
        assert_eq!(result.has_metric_sint64(), false);
    }
}

#[test]
fn events_to_message_test() {
    let mut e = event::Event::new();
    e.metric = Some(event::Metric::Double(10.1));
    let events = vec![e];
    let result = events_to_message(&events);
    assert_eq!(result.get_events().len(), 1);
    assert_eq!(result.get_events()[0].get_metric_d(), 10.1);

    let result = events_to_message(&vec![event::Event::new(), event::Event::new()]);
    assert_eq!(result.get_events().len(), 2);
}

#[test]
fn get_events_test() {
    let mut msg = proto::Msg::new();
    let e1 = proto::Event::new();
    let e2 = proto::Event::new();
    msg.set_events(RepeatedField::from_vec(vec![e1, e2]));
    let result = get_events(&msg);
    assert_eq!(result.len(), 2);
}

#[test]
fn proto_to_msg_test() {
    let mut msg = proto::Msg::new();
    let mut e1 = proto::Event::new();
    msg.set_ok(true);
    let mut query = proto::Query::new();
    query.set_string("true".to_owned());
    msg.set_query(query);
    e1.set_host("foo".to_owned());
    msg.set_events(RepeatedField::from_vec(vec![e1]));
    let result = proto_to_msg(&msg);
    assert_eq!(result.ok, Some(true));
    assert_eq!(result.query, Some("true".to_owned()));
    let events = result.events.unwrap();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.host, Some("foo".to_owned()));
}
