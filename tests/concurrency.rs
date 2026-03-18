use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

use tsl::{Event, TSL};

#[test]
fn test_concurrent_append() {
    let tsl: Arc<Mutex<TSL>> = Arc::new(Mutex::new(TSL::new(100)));

    let mut handles = vec![];

    for t in 0..4 {
        let tsl_clone = Arc::clone(&tsl);

        let handle = thread::spawn(move || {
            for i in 0..1000 {
                let mut tsl = tsl_clone.lock();
                tsl.append(Event::new(i + t * 1000, vec![1]));
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let tsl = tsl.lock();
    assert!(tsl.latest(1).len() > 0);
}
