
use clap;
use riemann_rust::util;
use riemann_rust::event::{Metric, Time, Event};
use std::collections::HashMap;

pub fn get_event(matches: &clap::ArgMatches) -> Result<Event, util::ParseError> {

    let host = matches.value_of("host").and_then(|s| Some(s.to_owned()));
    let service = matches.value_of("service").and_then(|s| Some(s.to_owned()));
    let state = matches.value_of("state").and_then(|s| Some(s.to_owned()));
    let description = matches.value_of("description").and_then(|s| Some(s.to_owned()));
    let ttl = util::parse_float(matches.value_of("ttl"))?;

    let time = util::parse_int64(matches.value_of("time"))?.and_then(|t| Some(Time::Seconds(t)));
    let tags = matches.values_of("tags").and_then(|tags| {
        let values: Vec<String> = tags.map(|t| t.to_owned()).collect();
        Some(values)
    });
    let attributes = matches.values_of("attributes").and_then(|attributes| {
        let mut result: HashMap<String, String> = HashMap::new();
        for attr in attributes {
            let kv: Vec<String> = attr.split(':').map(|v| v.to_owned()).collect();
            result.insert(kv[0].clone(), kv[1].clone());
        }
        Some(result)
    });

    // TODO : better error type
    let metric = {
        let test_int = util::parse_int64(matches.value_of("metric"));
        match test_int {
            Ok(m) => m.and_then(|m| Some(Metric::Int64(m))),
            Err(_) => {
                let test_double = util::parse_float64(matches.value_of("metric"))?;
                test_double.and_then(|m| Some(Metric::Double(m)))
            }
        }
    };

    Ok(Event {
        time: time,
        state: state,
        service: service,
        host: host,
        description: description,
        tags: tags,
        ttl: ttl,
        attributes: attributes,
        metric: metric
    })
}
