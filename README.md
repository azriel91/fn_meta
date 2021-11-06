# 🧬 FnMeta

[![Crates.io](https://img.shields.io/crates/v/fn_meta.svg)](https://crates.io/crates/fn_meta)
[![docs.rs](https://img.shields.io/docsrs/fn_meta)](https://docs.rs/fn_meta)
[![CI](https://github.com/azriel91/credent/workflows/CI/badge.svg)](https://github.com/azriel91/fn_meta/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/fn_meta/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/fn_meta)

Returns metadata about a function at runtime.

Currently this includes the [`TypeId`]s of function parameters.

This includes a [`FnMetadata`] struct and [`FnMetadataExt`] trait. `FnMetadataExt` adds the `.metadata()` function on functions and closures to return a `FnMetadata`, whose implementation returns function metadata at runtime.

## Usage

Add the following to `Cargo.toml`

```toml
fn_meta = "0.3.0"

# or
fn_meta = { version = "0.3.0", features = ["fn_meta_ext"] }
```

Code:

```rust
use core::any::TypeId;

use fn_meta::FnMetadataExt;

fn f1(_: &S0, _: &mut S1, _: &S2) -> () {}

let fn_metadata = f1.metadata();

assert_eq!(
    [TypeId::of::<S0>(), TypeId::of::<S2>()],
    fn_metadata.borrows()
);
assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts());

struct S0;
struct S1;
struct S2;
```

### Features

#### `"fn_meta_ext"`:

Enables the `FnMeta` and `FnMetaExt` traits. `FnMetaExt` adds the `.meta()` function on functions and closures to return a `Box<dyn FnMeta>`, which is the dynamic dispatch analog to `FnMetadata`.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[`TypeId`]: https://doc.rust-lang.org/std/any/struct.TypeId.html
[`FnMetadata`]: https://docs.rs/fn_meta/latest/fn_meta/struct.FnMetadata.html
[`FnMetadataExt`]: https://docs.rs/fn_meta/latest/fn_meta/struct.FnMetadataExt.html
[`FnMeta`]: https://docs.rs/fn_meta/latest/fn_meta/struct.FnMeta.html
[`FnMetaExt`]: https://docs.rs/fn_meta/latest/fn_meta/struct.FnMetaExt.html
