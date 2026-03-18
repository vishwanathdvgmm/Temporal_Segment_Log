use tsl::{Event, TSL};

#[test]
fn test_empty_query() {
    let tsl = TSL::new(10);
    let result = tsl.range_query(0, 100);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_single_event() {
    let mut tsl = TSL::new(10);

    tsl.append(Event::new(1, vec![1]));

    let result = tsl.range_query(1, 1);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_large_range() {
    let mut tsl = TSL::new(10);

    for i in 0..100 {
        tsl.append(Event::new(i, vec![1]));
    }

    let result = tsl.range_query(0, 100);
    assert_eq!(result.len(), 100);
}
