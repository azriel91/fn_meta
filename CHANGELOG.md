# Changelog

## 0.5.0 (2022-05-22)

* Use `smallvec::SmallVec` instead of `arrayvec::ArrayVec` to hold type IDs.

## 0.4.1 (2021-12-11)

* Remove `FnOnce` constraint for `Fun`.
* Rename `Args` type parameters to `ArgRefs`.
* Implement `FnMeta` for `*mut T`.
* Avoid stack overflow when borrows is empty.

## 0.4.0 (2021-11-07)

* Feature gate 7 and 8 arguments behind `"high_arg_count"` feature.

## 0.3.0 (2021-11-06)

* Rename `FnMetadata::reads` to `FnMetadata::borrows`.
* Rename `FnMetadata::writes` to `FnMetadata::borrow_muts`.
* Add `FnMetaExt` trait.
* Add `FnMeta` trait.
* Rename `FnMetadataExt::meta` to `FnMetadataExt::metadata`.
* Support 0 parameters.
* Support up to 8 parameters.
* Implement `FnMeta` for `Box<T>` where `T: FnMeta`.

## 0.2.0 (2021-10-25)

* `FnMetadata::reads` returns `[TypeId; N]` instead of `Vec<TypeId>`.
* `FnMetadata::writes` returns `[TypeId; N]` instead of `Vec<TypeId>`.

## 0.1.0 (2021-04-26)

* Add `FnMetadataExt` to allow querying `TypeId`s of function parameters.
