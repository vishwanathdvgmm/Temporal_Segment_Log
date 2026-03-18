use crate::event::Event;

pub struct Segment {
    pub events: Vec<Event>,
    pub start_time: u64,
    pub end_time: u64,
    capacity: usize,
}

impl Segment {
    pub fn new(capacity: usize, start_time: u64) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            start_time,
            end_time: start_time,
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.events.len() >= self.capacity
    }

    pub fn append(&mut self, event: Event) {
        self.end_time = event.timestamp;
        self.events.push(event);
    }
}
