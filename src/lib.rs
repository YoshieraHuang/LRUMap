mod cache;

use cache::*;
use std::collections::HashMap;
use std::hash::Hash;

pub use cache::Iter;

#[cfg(test)]
mod tests;

/// LRU map
#[derive(Debug, Default)]
pub struct LRUMap<K, T, const N: usize> {
    /// LRU Cache array
    cache: Cache<(K, T), N>,
    /// map to relate key and index in cache
    indices: HashMap<K, u16>,
}

impl<K, T, const N: usize> LRUMap<K, T, N>
where
    K:  Hash + Eq + Clone
{
    /// Put a key-value pair
    /// Returns the old value if the key exists, otherwise returns None
    pub fn put(&mut self, key: K, value: T) -> Option<T> {
        match self.indices.get(&key) {
            None => {
                // insert into cache and update the indices map
                self.cache.insert((key.clone(), value));
                self.indices.insert(key, self.cache.head);
                None
            },
            Some(idx) => {
                // just replace the value
                Some(self.cache.replace(*idx, (key, value)).1)
            }
        }
    }

    /// get the key-value pair and touch it
    pub fn get(&mut self, key: &K) -> Option<&T> {
        match self.indices.get(key) {
            None => None,
            Some(idx) => Some(&self.cache.get(*idx).1)
        }
    }

    /// Remove a key
    pub fn remove_one(&mut self, key: &K) {
        if let Some(idx) = self.indices.remove(key) {
            self.cache.remove(idx);
        }
    }

    /// Remove keys which match the predicate.
    pub fn remove<F>(&mut self, mut pred: F)
    where
        F: FnMut(&K) -> bool
    {
        // make a new hashmap and replace the old one to make the borrow checker happy
        let old_indices = std::mem::replace(&mut self.indices, HashMap::new());
        for (key, idx) in old_indices.into_iter() {
            if pred(&key) {
                self.cache.remove(idx);
            } else {
                self.indices.insert(key, idx);
            }
        }
    }

    /// Clear the LRU Cache
    #[inline]
    pub fn clear(&mut self) {
        self.indices.clear();
        self.cache.clear();
    }

    /// Number of items
    #[inline]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Touch the keys which match the predicate.
    pub fn touch<F>(&mut self, mut pred: F)
    where
        F: FnMut(&K) -> bool
    {
       for (key, idx) in self.indices.iter() {
            if pred(key) {
                self.cache.touch_index(*idx);
            }
        }
    }

    /// Iterator for keys and values
    pub fn iter(&self) -> Iter<(K, T), N> {
        self.cache.iter()
    }
}
