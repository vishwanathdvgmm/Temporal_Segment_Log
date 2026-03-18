use tsl::{Event, TSL};

#[test]
fn test_append_and_query() {
    let mut tsl = TSL::new(10);

    tsl.append(Event::new(1, vec![1]));
    tsl.append(Event::new(2, vec![2]));

    let result = tsl.range_query(1, 2);

    assert_eq!(result.len(), 2);
}
