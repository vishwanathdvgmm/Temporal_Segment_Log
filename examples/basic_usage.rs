use tsl::{Event, TSL};

fn main() {
    let mut tsl = TSL::new(100);

    tsl.append(Event::new(1, vec![10]));
    tsl.append(Event::new(2, vec![20]));

    let events = tsl.range_query(1, 2);

    println!("Found {} events", events.len());
}
