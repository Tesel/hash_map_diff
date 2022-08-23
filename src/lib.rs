use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct HashMapDiff<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub updated: HashMap<K, V>,
    pub removed: HashMap<K, V>,
}

pub fn hash_map_diff<K, V>(
    old_hash_map: &HashMap<K, V>,
    new_hash_map: &HashMap<K, V>,
) -> HashMapDiff<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + Clone,
{
    let mut removed: HashMap<K, V> = HashMap::new();
    for (key, value) in old_hash_map {
        if !new_hash_map.contains_key(key) {
            removed.insert(key.to_owned(), value.to_owned());
        }
    }

    let mut updated: HashMap<K, V> = HashMap::new();
    for (key, new_value) in new_hash_map {
        if let Some(old_value) = old_hash_map.get(key) {
            if new_value != old_value {
                updated.insert(key.to_owned(), new_value.to_owned());
            }
        } else {
            updated.insert(key.to_owned(), new_value.to_owned());
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
            removed: [("key1", 1)].into(),
        };
        assert_eq!(diff, expected_diff);
    }

    #[test]
    fn detect_changed_values() {
        let old_hash_map = [("key1", 1), ("key2", 2)].into();
        let new_hash_map = [("key1", 1), ("key2", 3)].into();
        let diff = hash_map_diff(&old_hash_map, &new_hash_map);
        let expected_diff = HashMapDiff {
            updated: [("key2", 3)].into(),
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
            updated: [("key3", 3)].into(),
            removed: HashMap::new(),
        };
        assert_eq!(diff, expected_diff);
    }
}
