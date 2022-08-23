# hash_map_diff

This library is intended to compute diff between two hashmaps to see which keys have been removed in lhs, and which ones have received new value in rhs.


## Usage example
```rust
use hash_map_diff::{hash_map_diff, HashMapDiff};

pub fn main() {
    let lhs = [("key1", 1), ("key2", 2), ("key3", 3)].into();
    let rhs = [("key1", 1), ("key3", 5), ("key4", 4)].into();

    let received_diff = hash_map_diff(&lhs, &rhs);

    let expected_diff = HashMapDiff {
        updated: [("key3", 5), ("key4", 4)].into(),
        removed: [("key2", 2)].into(),
    };

    assert_eq!(received_diff, expected_diff);
}

```


## Building
`cargo b`

## Testing
`cargo t`
