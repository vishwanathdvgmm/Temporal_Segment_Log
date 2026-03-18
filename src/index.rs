use std::collections::BTreeMap;

pub struct TemporalIndex {
    pub map: BTreeMap<u64, usize>,
}

impl TemporalIndex {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, timestamp: u64, segment_id: usize) {
        self.map.insert(timestamp, segment_id);
    }

    pub fn range(&self, start: u64, end: u64) -> Vec<usize> {
        self.map.range(start..=end).map(|(_, &id)| id).collect()
    }
}

impl Default for TemporalIndex {
    fn default() -> Self {
        Self::new()
    }
}
