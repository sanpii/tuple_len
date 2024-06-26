# tuple_len

[![Github actions](https://github.com/sanpii/tuple_len/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/tuple_len/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Gitlab CI](https://gitlab.com/sanpi/tuple_len/badges/main/pipeline.svg)](https://gitlab.com/sanpi/tuple_len/commits/main)

Get the number of elements in a tuple.

## Usage

Add it to your dependencies:

```sh
cargo add tuple_len
```

```rust
// The macro way, compute at compilation time
assert_eq!(tuple_len::tuple_len!(("hello", 5, 'c')), 3);

// The trait way — limited to sized 12 tuple
use tuple_len::TupleLen;
assert_eq!(().len(), 0);

// The function way — idem
let tuple = (1,);
assert_eq!(tuple_len::len(&tuple), 1);
```
