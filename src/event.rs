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
pub struct MsgError {
    pub message: String
}

impl Error for MsgError {
    fn description(&self) -> &str {
        "Error sending events to Riemann"
    }
}

impl fmt::Display for MsgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error sending events to Riemann : {}", self.message)
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
