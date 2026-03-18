use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

use crate::event::Event;
use crate::tsl::TSL;

pub fn concurrent_append(tsl: Arc<Mutex<TSL>>, threads: usize, per_thread: u64) {
    let mut handles = vec![];

    for t in 0..threads {
        let tsl_clone = Arc::clone(&tsl);

        let handle = thread::spawn(move || {
            for i in 0..per_thread {
                let mut tsl = tsl_clone.lock();
                tsl.append(Event::new(i + t as u64 * per_thread, vec![1]));
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
