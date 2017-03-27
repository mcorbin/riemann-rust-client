use proto::proto;
use protobuf::{RepeatedField};
use event;

pub fn event_to_proto(event: &event::Event) -> proto::Event {
    let mut e = proto::Event::new();
    if let Some(ref time) = event.time {
        match *time {
            event::Time::Seconds(s) => {
                e.set_time(s);
                e.set_time_micros(s * 1000000);
            }
            event::Time::Micros(s) =>  {
                e.set_time_micros(s);
                e.set_time(s/1000000) // compatibility with old Riemann server
            }
        }
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

#[cfg(test)]
mod tests {
    use super::event_to_proto;
    use std::collections::HashMap;
    use proto::proto;
    use event;

    #[test]
    fn event_to_proto_test() {
        let mut attr = HashMap::new();
        attr.insert("foo".to_owned(), "bar".to_owned());
        let e = event::Event {
            time: Some(event::Time::Seconds(1)),
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
        assert_eq!(result.get_time_micros(), 1000000);
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
        e.time = Some(event::Time::Micros(10));
        let result = event_to_proto(&e);
        assert_eq!(result.get_time(), 0);
        assert_eq!(result.get_time_micros(), 10);

        let mut e = event::Event::new();
        e.time = Some(event::Time::Seconds(10));
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

