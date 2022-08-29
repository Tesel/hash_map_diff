use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct HashMapDiff<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub updated: HashMap<K, V>,
    pub removed: HashMap<K, V>,
}

pub fn hash_map_diff<'a, K, V>(
    lhs: &'a HashMap<K, V>,
    rhs: &'a HashMap<K, V>,
) -> HashMapDiff<&'a K, &'a V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + Clone,
{
    let mut removed: HashMap<&'a K, &'a V> = HashMap::new();
    for (key, value) in lhs {
        if !rhs.contains_key(key) {
            removed.insert(key, value);
        }
    }

    let mut updated: HashMap<&'a K, &'a V> = HashMap::new();
    for (key, new_value) in rhs {
        if let Some(old_value) = lhs.get(key) {
            if new_value != old_value {
                updated.insert(key, new_value);
            }
        } else {
            updated.insert(key, new_value);
        }
    }

    HashMapDiff { updated, removed }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn detect_removed_elements() {
        let old_hash_map = [("key1", 1), ("key2", 2)].into();
        let new_hash_map = [("key2", 2)].into();
        let diff = hash_map_diff(&old_hash_map, &new_hash_map);
        let expected_diff = HashMapDiff {
            updated: HashMap::new(),
            removed: [(&"key1", &1)].into(),
        };
        assert_eq!(diff, expected_diff);
    }

    #[test]
    fn detect_changed_values() {
        let old_hash_map = [("key1", 1), ("key2", 2)].into();
        let new_hash_map = [("key1", 1), ("key2", 3)].into();
        let diff = hash_map_diff(&old_hash_map, &new_hash_map);
        let expected_diff = HashMapDiff {
            updated: [(&"key2", &3)].into(),
            removed: HashMap::new(),
        };
        assert_eq!(diff, expected_diff);
    }

    #[test]
    fn detect_new_keys() {
        let old_hash_map = [("key1", 1)].into();
        let new_hash_map = [("key1", 1), ("key3", 3)].into();
        let diff = hash_map_diff(&old_hash_map, &new_hash_map);
        let expected_diff = HashMapDiff {
            updated: [(&"key3", &3)].into(),
            removed: HashMap::new(),
        };
        assert_eq!(diff, expected_diff);
    }

    #[test]
    fn usage_test() {
        let lhs = [("unchanged", 1), ("removed", 2), ("changed", 3)].into();
        let rhs = [("unchanged", 1), ("changed", 5), ("added", 4)].into();

        let received_diff = hash_map_diff(&lhs, &rhs);

        let expected_diff = HashMapDiff {
            updated: [(&"changed", &5), (&"added", &4)].into(),
            removed: [(&"removed", &2)].into(),
        };

        assert_eq!(received_diff, expected_diff);
    }
}
