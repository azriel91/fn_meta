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
//!     fn_metadata.borrows()
//! );
//! assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts());
//! #
//! # struct S0;
//! # struct S1;
//! # struct S2;
//! ```

pub use crate::{fn_metadata::FnMetadata, into_fn_metadata::IntoFnMetadata};

mod fn_metadata;
mod into_fn_metadata;

#[cfg(feature = "fn_meta")]
pub use crate::{fn_meta::FnMeta, into_fn_meta::IntoFnMeta, type_ids::TypeIds};

#[cfg(feature = "fn_meta")]
mod fn_meta;
#[cfg(feature = "fn_meta")]
mod into_fn_meta;
#[cfg(feature = "fn_meta")]
mod type_ids;
