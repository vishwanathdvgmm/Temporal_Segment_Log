# Temporal Segment Log (TSL)

## Overview

TSL is a hybrid data structure optimized for high-throughput time-ordered event streams.

## Features

- O(1) append
- O(log n + k) range queries
- Segment-based retention
- Concurrent-safe design

## Use Cases

- Real-time analytics
- AI pipelines
- Observability systems

## Example

```rust
let mut tsl = TSL::new(100);

tsl.append(Event::new(1, vec![1,2,3]));
let results = tsl.range_query(0, 10);
```

## Architecture

- Append-only segments
- Temporal index (BTree)
- Concurrent access
