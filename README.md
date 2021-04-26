# 🧬 FnMeta

[![Crates.io](https://img.shields.io/crates/v/fn_meta.svg)](https://crates.io/crates/fn_meta)
[![docs.rs](https://docs.rs/fn_meta/badge.svg)](https://docs.rs/fn_meta)
![CI](https://github.com/azriel91/fn_meta/workflows/CI/badge.svg)
[![Coverage Status](https://codecov.io/gh/azriel91/fn_meta/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/fn_meta)

Returns metadata about a function.

# Examples

```rust
use fn_meta::FnMetadataExt;

fn my_function(_: &S0, _: &mut S1, _: &S2) -> () {}

let fn_metadata = my_function.meta();

assert_eq!(
    alloc::vec![TypeId::of::<S0>(), TypeId::of::<S2>()],
    fn_metadata.reads()
);
assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.writes());

struct S0;
struct S1;
struct S2;
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
