#[derive(Clone, Debug)]
pub struct Event {
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

impl Event {
    pub fn new(timestamp: u64, payload: Vec<u8>) -> Self {
        Self { timestamp, payload }
    }
}
