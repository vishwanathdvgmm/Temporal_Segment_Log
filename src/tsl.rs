use parking_lot::Mutex;
use std::sync::Arc;

use crate::event::Event;
use crate::index::TemporalIndex;
use crate::segment::Segment;

pub struct TSL {
    pub segments: Vec<Arc<Mutex<Segment>>>,
    pub index: TemporalIndex,
    pub active_segment: usize,
    pub segment_capacity: usize,
}

impl TSL {
    pub fn new(segment_capacity: usize) -> Self {
        let initial = Segment::new(segment_capacity, 0);

        Self {
            segments: vec![Arc::new(Mutex::new(initial))],
            index: TemporalIndex::new(),
            active_segment: 0,
            segment_capacity,
        }
    }

    pub fn append(&mut self, event: Event) {
        let idx = self.active_segment;

        // Scope to drop lock early
        let is_full = {
            let seg = self.segments[idx].lock();
            seg.is_full()
        };

        if is_full {
            let new_idx = self.segments.len();

            let new_segment = Segment::new(self.segment_capacity, event.timestamp);

            self.segments.push(Arc::new(Mutex::new(new_segment)));
            self.index.insert(event.timestamp, new_idx);

            self.active_segment = new_idx;

            let mut seg = self.segments[new_idx].lock();
            seg.append(event);
        } else {
            let mut seg = self.segments[idx].lock();
            seg.append(event);
        }
    }

    pub fn range_query(&self, start: u64, end: u64) -> Vec<Event> {
        let mut result = Vec::new();

        for seg_id in self.index.range(start, end) {
            let seg = self.segments[seg_id].lock();

            for e in &seg.events {
                if e.timestamp >= start && e.timestamp <= end {
                    result.push(e.clone());
                }
            }
        }

        result
    }

    pub fn latest(&self, n: usize) -> Vec<Event> {
        let seg = self.segments[self.active_segment].lock();
        seg.events.iter().rev().take(n).cloned().collect()
    }
}
