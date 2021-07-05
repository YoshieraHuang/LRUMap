use super::*;

type TestMap = LRUMap<&'static str, i32, 4>;

fn items<K, T, const N: usize>(map: &mut LRUMap<K, T, N>) -> Vec<(K, T)>
where
    K: Hash + Eq + Clone, 
    T: Clone
{
    let mut v = Vec::new();
    for (k, val) in map.iter() {
        v.push((k.clone(), val.clone()))
    }
    v
}

/// Get the reference of the value and remain it untouched
#[inline]
fn get_untouch<T, const N: usize>(cache: &Cache<T, N>, idx: u16) -> &T {
    &cache.entries[idx as usize].val
}

/// Check whether the keys in `indices` match the keys in `cache`
/// Used for testing.  
fn check_keys<K, T, const N: usize>(map: &LRUMap<K, T, N>) -> bool
where
    K: Hash + Eq + Clone, 
    T: Clone
{
    for (key, idx) in map.indices.iter() {
        if get_untouch(&map.cache, *idx).0 != *key {
            return false
        }
    }
    return true
}

#[test]
fn empty() {
    let cache = TestMap::default();
    assert!(cache.len() == 0);
}

#[test]
fn put() {
    let mut cache = TestMap::default();
    assert_eq!(cache.put("1", 1), None);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.put("2", 2), None);
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.put("3", 3), None);
    assert_eq!(cache.len(), 3);
    assert_eq!(cache.put("4", 4), None);
    assert_eq!(cache.len(), 4);
    assert_eq!(
        items(&mut cache),
        [("4", 4), ("3", 3), ("2", 2), ("1", 1)],
        "Ordered from most- to least_recent."
    );
    assert!(check_keys(&cache), "check keys");

    assert_eq!(cache.put("5", 5), None);
    assert_eq!(cache.len(), 4);
    assert_eq!(
        items(&mut cache),
        [("5", 5), ("4", 4), ("3", 3), ("2", 2)],
        "Least-recently-used item evicted."
    );

    assert_eq!(cache.put("6", 6), None);
    assert_eq!(cache.put("7", 7), None);
    assert_eq!(cache.put("8", 8), None);
    assert_eq!(cache.put("9", 9), None);
    assert_eq!(
        items(&mut cache),
        [("9", 9), ("8", 8), ("7", 7), ("6", 6)],
        "Least-recently-used item evicted."
    );

    assert_eq!(cache.put("7", 14), Some(7));
    assert_eq!(
        items(&mut cache),
        [("7", 14), ("9", 9), ("8", 8), ("6", 6)],
        "Least-recently-used item evicted."
    );
}

#[test]
fn cache_hit() {
    let mut cache = TestMap::default();

    assert_eq!(cache.put("1", 1), None);
    assert_eq!(cache.put("2", 2), None);
    assert_eq!(cache.put("3", 3), None);
    assert_eq!(cache.put("4", 4), None);
    assert_eq!(cache.len(), 4);
    assert_eq!(
        items(&mut cache),
        [("4", 4), ("3", 3), ("2", 2), ("1", 1)],
        "Ordered from most- to least-recent."
    );

    assert_eq!(cache.get(&"1"), Some(&1));
    assert_eq!(
        items(&mut cache),
        [("1", 1), ("4", 4), ("3", 3), ("2", 2)],
        "Matching item moved to front."
    );

    cache.remove_one(&"2");
    assert_eq!(
        items(&mut cache),
        [("1", 1), ("4", 4), ("3", 3)],
        "Removing one item."
    );

    cache.touch(|k| *k == "3");
    assert_eq!(
        items(&mut cache),
        [("3", 3), ("1", 1), ("4", 4)],
        "Touching one item."
    );
}

#[test]
fn clear() {
    let mut cache = TestMap::default();
    cache.put("1", 1);
    cache.clear();
    assert_eq!(items(&mut cache), [], "all items evicted");

    cache.put("1", 1);
    cache.put("2", 2);
    cache.put("3", 3);
    cache.put("4", 4);
    assert_eq!(
        items(&mut cache),
        [("4", 4), ("3", 3), ("2", 2), ("1", 1)]);
    cache.clear();
    assert_eq!(items(&mut cache), [], "all items evicted again");
}
