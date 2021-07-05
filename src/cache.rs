#![deny(unsafe_code)]

//! A simple, fast, least-recently-used (LRU) cache.

extern crate arrayvec;

use arrayvec::ArrayVec;

/// A LRU Cache based on array
#[derive(Debug)]
pub(crate) struct Cache<T, const N: usize> {
    // staticaly-sized array on which linked list is built
    pub(crate) entries: ArrayVec<Entry<T>, N>,
    // Index of the most-recently-used entry
    pub(crate) head: u16,
    // Index of the least-recently-used entry
    pub(crate) tail: u16
}

// An entry in the cache
#[derive(Debug, Clone)]
pub(crate) struct Entry<T> {
    pub(crate) val: T,
    /// Index of the previous entry
    prev: u16,
    /// Index of the next entry
    next: u16,
}

impl<T, const N: usize> Default for Cache<T, N> {
    fn default() -> Self {
        let cache = Cache {
            entries: ArrayVec::new(),
            head: 0,
            tail: 0,
        };
        assert!(
            cache.entries.capacity() < u16::max_value() as usize,
            "Capacity overflow"
        );
        cache
    }
}

impl<T, const N: usize> Cache<T, N> {
    /// Insert an item in the cache
    /// 
    /// This item becomes most-recently-used item.
    /// If the cache is full, the least-recently-used item will be removed.
    pub(crate) fn insert(&mut self, val: T) {
        let entry = Entry {
            val,
            prev: 0,
            next: 0,
        };

        let new_head = if self.entries.len() == self.entries.capacity() {
            let i = self.pop_back();
            self.entries[i as usize] = entry;
            i
        } else {
            self.entries.push(entry);
            self.entries.len() as u16 -1
        };

        self.push_front(new_head);
    }

    /// Touch a given entry, putting it first in the list
    #[inline]
    pub(crate) fn touch_index(&mut self, idx: u16) {
        if idx != self.head {
            self.remove(idx);
            self.push_front(idx);
        }
    }

    /// Returns the number of elements in the cache
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    /// Evict all elements from the cache
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    /// Remove an entry from the linked list
    pub(crate) fn remove(&mut self, idx: u16) {
        let prev = self.entries[idx as usize].prev;
        let next = self.entries[idx as usize].next;

        if idx == self.head {
            self.head = next;
        } else {
            self.entries[prev as usize].next = next;
        }

        if idx == self.tail {
            self.tail = prev;
        } else {
            self.entries[next as usize].prev = prev;
        }
    }

    /// Insert a new entry at `idx` at the head of the list
    pub(crate) fn push_front(&mut self, idx: u16) {
        if self.entries.len() == 1 {
            self.tail = idx;
        } else {
            self.entries[idx as usize].next = self.head;
            self.entries[self.head as usize].prev = idx;
        }
        self.head = idx;
    }

    /// Remove the last entry from the linked list.
    /// Returns the index of the removed entry
    pub(crate) fn pop_back(&mut self) -> u16 {
        let old_tail = self.tail;
        let new_tail = self.entries[old_tail as usize].prev;
        self.tail = new_tail;
        old_tail
    }

    /// Replace the item in the linked list.
    /// Returns the replaced item.
    pub(crate) fn replace(&mut self, idx: u16, val: T) -> T {
        self.touch_index(idx);
        let entry = &mut self.entries[idx as usize];
        std::mem::replace(&mut entry.val, val)
    }

    /// Touch the index and get the reference of the value
    pub(crate) fn get(&mut self, idx: u16) -> &T {
        self.touch_index(idx);
        &self.entries[idx as usize].val
    }

    pub(crate) fn iter(&self) -> Iter<T, N> {
        Iter {
            cache: self,
            pos: self.head,
        }
    }
}

impl<T, const N: usize> Clone for Cache<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            head: self.head,
            tail: self.tail,
        }
    }
}

/// Iterator over values in an LRUCache, from most-recently-used to least-recently-used.
pub struct Iter<'a, T, const N: usize> {
    cache: &'a Cache<T, N>,
    pos: u16,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let entry = self.cache.entries.get(self.pos as usize)?;

        self.pos = if self.pos == self.cache.tail {
            N as u16 // Point past the end of the array to signal we are done.
        } else {
            entry.next
        };
        Some(&entry.val)
    }
}