# LRU Cache

## Overview

The implementation focuses on:

- Correct LRU semantics
- Thread safety
- Reasonable performance
- Simplicity and clarity

The cache supports the following operations:

- `put(key, value)` - insert or update a value
- `get(key)` - retrieve a value and mark it as recently used
- `len()` - number of elements currently stored
- `is_empty()` - whether the cache is empty

## Data Structures

- `HashMap<K, V>` — stores key-value pairs
- `VecDeque<K>` — tracks usage order
  - Front = Least Recently Used
  - Back = Most Recently Used

## Synchronization Strategy

Concurrency is handled using:

RwLock (inner: RwLock<CacheState<K, V>>)

Protects the cache state (HashMap + VecDeque).

Allows multiple concurrent readers (for len, is_empty) and exclusive access for writers (for get and put).

Simplifies thread safety without requiring fine-grained locks for HashMap and VecDeque.

## Known Limitations

- Performance under high concurrency:

get requires a write lock to maintain LRU ordering, which can become a bottleneck with many readers.

- VecDeque removal cost:

Removing an element from the middle of VecDeque is O(n). For large caches, this can be inefficient.

- No time-based expiration:

LRU is purely based on access order, not time.

- Blocking behavior:

A single write lock blocks other operations; under heavy load, this may reduce throughput.
