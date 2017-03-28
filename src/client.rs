use event;

trait Client {
    fn send_event(&self, event: event::Event) -> Option<event::Message>;
    fn send_events(&self, events: &Vec<event::Event>) -> Option<event::Message>;
    fn query(&self, query: &event::Query) -> Option<event::Message>;
}
