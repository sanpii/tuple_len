# tuple_len

[![Github actions](https://github.com/sanpii/tuple_len/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/tuple_len/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Gitlab CI](https://gitlab.com/sanpi/tuple_len/badges/main/pipeline.svg)](https://gitlab.com/sanpi/tuple_len/commits/main)

Get the number of elements in a tuple.

## Usage

Add it to your dependencies:

```toml
[dependencies]
tuple_len = "1.0"
```

```rust
// The macro way, compute at compilation time
assert_eq!(tuple_len::tuple_len!(("hello", 5, 'c')), 3);

// The trait way
use tuple_len::TupleLen;
assert_eq!(().len(), 0);

// The function way
let tuple = (1,);
assert_eq!(tuple_len::len(&tuple), 1);
```
