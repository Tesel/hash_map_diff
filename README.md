# hash_map_diff

This library is intended to compute diff between two hashmaps to see which keys have been removed in lhs, and which ones have received new value in rhs.


## Usage example
```rust
use hash_map_diff::{hash_map_diff, HashMapDiff};

pub fn main() {
    let lhs = [("unchanged", 1), ("removed", 2), ("changed", 3)].into();
    let rhs = [("unchanged", 1), ("changed", 5), ("added", 4)].into();

    let received_diff = hash_map_diff(&lhs, &rhs);

    let expected_diff = HashMapDiff {
        updated: [(&"changed", &5), (&"added", &4)].into(),
        removed: [(&"removed", &2)].into(),
    };

    assert_eq!(received_diff, expected_diff);
}

```


## Building
`cargo b`

## Testing
`cargo t`
