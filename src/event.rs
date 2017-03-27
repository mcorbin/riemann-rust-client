use std::collections::HashMap;

pub enum Metric {
    Int64(i64),
    Double(f64),
    Float(f32)
}

pub enum Time {
    Seconds(i64),
    Micros(i64)
}

pub struct Event {
    pub time: Option<Time>,
    pub state: Option<String>,
    pub service: Option<String>,
    pub host: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ttl: Option<f32>,
    pub attributes: Option<HashMap<String, String>>,
    pub metric: Option<Metric>

}

pub type Query = String;

pub struct Message {
    pub ok: Option<bool>,
    pub error: Option<String>,
    pub query: Query,
    pub events: Vec<Event>
}

impl Event {
    pub fn new() -> Event {
        Event {
            time: None,
            state: None,
            service: None,
            host: None,
            description: None,
            tags: None,
            ttl: None,
            attributes: None,
            metric: None
        }
    }
}
