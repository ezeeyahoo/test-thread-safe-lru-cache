use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::RwLock;

pub struct LruCache<K, V> {
    capacity: usize,
    inner: RwLock<CacheState<K, V>>,
}

struct CacheState<K, V> {
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        Self {
            capacity,
            inner: RwLock::new(CacheState {
                map: HashMap::with_capacity(capacity),
                order: VecDeque::with_capacity(capacity),
            }),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut state = self.inner.write().unwrap();

        if let Some(value) = state.map.get(key).cloned() {
            // most recently used
            if let Some(pos) = state.order.iter().position(|k| k == key) {
                state.order.remove(pos);
            }
            state.order.push_back(key.clone());

            Some(value)
        } else {
            None
        }
    }

    pub fn put(&self, key: K, value: V) {
        let mut state = self.inner.write().unwrap();

        if state.map.contains_key(&key) {
            state.map.insert(key.clone(), value);

            if let Some(pos) = state.order.iter().position(|k| k == &key) {
                state.order.remove(pos);
            }

            state.order.push_back(key);
            return;
        }

        if state.map.len() == self.capacity {
            if let Some(lru_key) = state.order.pop_front() {
                state.map.remove(&lru_key);
            }
        }

        state.map.insert(key.clone(), value);
        state.order.push_back(key);
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn basic_put_get() {
        let cache = LruCache::new(2);

        cache.put(1, "a");
        cache.put(2, "b");

        assert_eq!(cache.get(&1), Some("a"));
        assert_eq!(cache.get(&2), Some("b"));
    }

    #[test]
    fn eviction_works() {
        let cache = LruCache::new(2);

        cache.put(1, "a");
        cache.put(2, "b");

        cache.get(&1);
        cache.put(3, "c");

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some("a"));
        assert_eq!(cache.get(&3), Some("c"));
    }

    #[test]
    fn concurrent_access() {
        let cache = Arc::new(LruCache::new(50));
        let mut handles = vec![];

        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);

            handles.push(thread::spawn(move || {
                for j in 0..1000 {
                    cache_clone.put(j, i);
                    cache_clone.get(&j);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert!(cache.len() <= 50);
    }
}
