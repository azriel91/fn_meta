#![no_std]

//! Returns metadata about a function at runtime.
//!
//! Currently this includes the [`TypeId`]s of function parameters.
//!
//! This includes a [`FnMetadata`] struct and [`FnMetadataExt`] trait.
//! `FnMetadataExt` adds the `.metadata()` function on functions and closures to
//! return a `FnMetadata`, whose implementation returns function metadata at
//! runtime.
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! fn_meta = "0.6.0"
//!
//! # or
//! fn_meta = { version = "0.6.0", features = ["fn_meta_ext"] }
//! ```
//!
//! Code:
//!
//! ```rust
//! # use core::any::TypeId;
//! #
//! use fn_meta::{FnMetaDyn, FnMetadataExt};
//!
//! fn f1(_: &S0, _: &mut S1, _: &S2) -> () {}
//!
//! let fn_metadata = f1.metadata();
//!
//! assert_eq!(
//!     [TypeId::of::<S0>(), TypeId::of::<S2>()],
//!     fn_metadata.borrows().as_slice()
//! );
//! assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts().as_slice());
//! #
//! # struct S0;
//! # struct S1;
//! # struct S2;
//! ```
//!
//! ### Features
//!
//! #### `"fn_meta_ext"`:
//!
//! Enables the [`FnMeta`] and [`FnMetaExt`] traits. `FnMetaExt` adds the
//! `.meta()` function on functions and closures to return a `Box<dyn FnMeta>`,
//! which is the dynamic dispatch analog to `FnMetadata`.
//!
//! #### `"high_arg_count"`:
//!
//! Raises the number of arguments that [`FnMetaExt`] and [`FnMetadataExt`] are
//! implemented for from 6 to 8.
//!
//! This is feature gated because compilation time increasing significantly with
//! higher numbers of arguments -- as much as from 1.5 seconds for 6 arguments
//! to 8 seconds for 8 arguments.
//!
//! [`TypeId`]: core::any::TypeId
//! [`FnMetadata`]: crate::FnMetadata
//! [`FnMetadataExt`]: crate::FnMetadataExt
//! [`FnMeta`]: crate::FnMeta
//! [`FnMetaExt`]: crate::FnMetaExt

pub use crate::{fn_metadata::FnMetadata, fn_metadata_ext::FnMetadataExt};

mod fn_metadata;
mod fn_metadata_ext;

#[cfg(feature = "fn_meta_ext")]
pub use crate::{
    fn_meta::{FnMeta, FnMetaDyn},
    fn_meta_ext::FnMetaExt,
    type_ids::TypeIds,
};

#[cfg(feature = "fn_meta_ext")]
mod fn_meta;
#[cfg(feature = "fn_meta_ext")]
mod fn_meta_ext;
#[cfg(feature = "fn_meta_ext")]
mod type_ids;
