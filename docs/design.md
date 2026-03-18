# Design

## Core Idea

TSL combines:

- append-only segments
- temporal index
- concurrent-safe ingestion

## Invariants

- monotonic timestamps
- non-overlapping segments

## Complexity

- append: O(1)
- query: O(log n + k)
