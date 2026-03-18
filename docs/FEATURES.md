# **Temporal Segment Log (TSL)**

A hybrid data structure designed for high-throughput time-ordered data.

Conceptually it combines:

- append-only log
- segmented arrays
- time index tree

## **I. Design Choice**

**Target Environment**

`Distributed + parallel systems`

Reasoning:

- streaming workloads are typically distributed
- parallel writes are required
- CPU cache locality still matters

## **II. Structural Layout**

```
Temporal Segment Log

Segment 1
[ t1 | data ]
[ t2 | data ]
[ t3 | data ]

Segment 2
[ t4 | data ]
[ t5 | data ]

Segment 3
[ t6 | data ]
[ t7 | data ]

Index Tree
t1 ----> segment1
t4 ----> segment2
t6 ----> segment3
```

Components:

1. **Segments**
    - fixed-size append blocks
    - optimized for sequential writes

2. **Time Index Tree**

    balanced tree mapping timestamps → segments

3. **Hot Cache**

    in-memory window for most recent events

## **III. Supported Operations**

| Operation          | Description                 | Target Complexity |
| ------------------ | --------------------------- | ----------------- |
| append(event)      | add new event               | O(1)              |
| query_range(t1,t2) | fetch events in time window | O(log n + k)      |
| latest(n)          | last N events               | O(1)              |
| delete_before(t)   | garbage collect old data    | O(log n)          |
| parallel_read      | multiple consumers          | lock-free         |

## **IV. Invariants**

1. Events are **monotonically ordered by timestamp.**
2. Each segment has **fixed capacity.**
3. Index tree maintains **segment boundaries.**

## **V. Advantages**

Compared with existing solutions:

| Feature              | Benefit                      |
| -------------------- | ---------------------------- |
| append-only segments | fast ingestion               |
| time index           | fast temporal queries        |
| segment deletion     | efficient retention policies |
| lock-free append     | high concurrency             |

## **VI. Potential Industry Uses**

- **AI training pipelines**
- **event sourcing systems**
- **real-time analytics engines**
- **distributed observability platforms**
- **financial streaming systems**

## **VII. Example API**

```rust
tsl.append(event)

tsl.query_range(start_time, end_time)

tsl.latest(100)

tsl.delete_before(expiration_time)
```
