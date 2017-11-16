use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum Metric {
    Int64(i64),
    Double(f64),
    Float(f32)
}

type State = String;
type Service = String;
type Host = String;
type Description = String;
type Tag = String;
type Tags = Vec<Tag>;
type Ttl = f32;
type AttrKey = String;
type AttrValue = String;
type Attributes = HashMap<AttrKey, AttrValue>;

#[derive(Debug)]
pub struct Event {
    pub time: Option<DateTime<Utc>>,
    pub state: Option<State>,
    pub service: Option<Service>,
    pub host: Option<Host>,
    pub description: Option<Description>,
    pub tags: Option<Tags>,
    pub ttl: Option<Ttl>,
    pub attributes: Option<Attributes>,
    pub metric: Option<Metric>
}

pub type Query = String;

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

#[derive(Debug)]
pub struct Msg {
    pub ok: Option<bool>,
    pub error: Option<String>,
    pub query: Option<Query>,
    pub events: Option<Vec<Event>>
}

impl Msg {
    pub fn new() -> Msg {
        Msg {
            ok: None,
            error: None,
            query: None,
            events: None
        }
    }
}

impl Error for Msg {
    fn description(&self) -> &str {
        let error_msg = match self.error {
            Some(ref e) => e,
            None => "Unknown error"
        };
        error_msg
    }
}

impl fmt::Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match self.error {
            Some(ref e) => e,
            None => "Unknown error"
        };
        write!(f, "{}", error_msg)
    }
}

#[derive(Debug)]
pub struct RiemannClientError {
    pub message: String
}

impl Error for RiemannClientError {
    fn description(&self) -> &str {
        "Error with the Riemann client"
    }
}

impl fmt::Display for RiemannClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error with the Riemann client : {}", self.message)
    }
}
