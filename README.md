# tuple_len

[![Github actions](https://github.com/sanpii/tuple_len/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/tuple_len/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Gitlab CI](https://gitlab.com/sanpi/tuple_len/badges/main/pipeline.svg)](https://gitlab.com/sanpi/tuple_len/commits/main)

## Usage

Add it to your dependencies:

```toml
[dependencies]
tuple_len = "1.0"
```

```rust
tuple_len!(("hello", 5, 'c')) == 3
```
