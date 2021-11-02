#![no_std]

//! Returns metadata about a function.
//!
//! # Examples
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! fn_meta = "0.2.0"
//! ```
//!
//! Code:
//!
//! ```rust
//! # use core::any::TypeId;
//! #
//! use fn_meta::IntoFnMetadata;
//!
//! fn my_function(_: &S0, _: &mut S1, _: &S2) -> () {}
//!
//! let fn_metadata = my_function.into_fn_metadata();
//!
//! assert_eq!(
//!     [TypeId::of::<S0>(), TypeId::of::<S2>()],
//!     fn_metadata.reads()
//! );
//! assert_eq!([TypeId::of::<S1>()], fn_metadata.writes());
//! #
//! # struct S0;
//! # struct S1;
//! # struct S2;
//! ```

pub use crate::{fn_metadata::FnMetadata, into_fn_metadata::IntoFnMetadata};

mod fn_metadata;
mod into_fn_metadata;

#[cfg(test)]
mod tests {
    use core::any::TypeId;

    use super::IntoFnMetadata;

    #[test]
    fn read_1_write_1() {
        let fn_metadata = f_r1_w1.into_fn_metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.reads());
        assert_eq!([TypeId::of::<S1>()], fn_metadata.writes());
    }

    #[test]
    fn write_1_read_1() {
        let fn_metadata = f_w1_r1.into_fn_metadata();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.reads());
        assert_eq!([TypeId::of::<S0>()], fn_metadata.writes());
    }

    #[test]
    fn read_1_write_1_read_1() {
        let fn_metadata = f_r1_w1_r1.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.reads()
        );
        assert_eq!([TypeId::of::<S1>()], fn_metadata.writes());
    }

    #[test]
    fn write_1_read_1_write_1() {
        let fn_metadata = f_w1_r1_w1.into_fn_metadata();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.reads());
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_2_read_2_write_2_read_1() {
        let fn_metadata = f_w2_r2_w2_r1.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            fn_metadata.reads()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.writes()
        );
    }

    #[test]
    fn read_1() {
        let fn_metadata = f_r1.into_fn_metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.reads());
    }

    #[test]
    fn read_2() {
        let fn_metadata = f_r2.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_3() {
        let fn_metadata = f_r3.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_4() {
        let fn_metadata = f_r4.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_5() {
        let fn_metadata = f_r5.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_6() {
        let fn_metadata = f_r6.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.reads()
        );
    }

    #[test]
    fn write_1() {
        let fn_metadata = f_w1.into_fn_metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.writes());
    }

    #[test]
    fn write_2() {
        let fn_metadata = f_w2.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_3() {
        let fn_metadata = f_w3.into_fn_metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_4() {
        let fn_metadata = f_w4.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_5() {
        let fn_metadata = f_w5.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_6() {
        let fn_metadata = f_w6.into_fn_metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.writes()
        );
    }

    #[cfg(not(tarpaulin_include))]
    fn f_r1(_: &S0) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r2(_: &S0, _: &S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r3(_: &S0, _: &S1, _: &S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r4(_: &S0, _: &S1, _: &S2, _: &S3) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r5(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r6(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5) -> () {}

    #[cfg(not(tarpaulin_include))]
    fn f_w1(_: &mut S0) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w2(_: &mut S0, _: &mut S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w3(_: &mut S0, _: &mut S1, _: &mut S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w4(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w5(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w6(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4, _: &mut S5) -> () {}

    #[cfg(not(tarpaulin_include))]
    fn f_r1_w1(_: &S0, _: &mut S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w1_r1(_: &mut S0, _: &S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r1_w1_r1(_: &S0, _: &mut S1, _: &S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w1_r1_w1(_: &mut S0, _: &S1, _: &mut S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w2_r2_w2_r1(_: &mut S0, _: &mut S1, _: &S2, _: &S3, _: &mut S4, _: &mut S5, _: &S6) -> () {
    }

    struct S0;
    struct S1;
    struct S2;
    struct S3;
    struct S4;
    struct S5;
    struct S6;
}
