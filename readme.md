# Bucketizer

```rust
extern crate bucketizer;

use bucketizer::Bucketizer;

fn main() {
    let mut b = Bucketizer::new();
    b.bucket(Some(0.0), Some(5.0), 2.5);
    assert_eq!(b.bucketize(0.1), Some(2.5));
}
```
