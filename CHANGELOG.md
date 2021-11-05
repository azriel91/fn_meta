# Changelog

## 0.3.0 (unreleased)

* Rename `FnMetadata::reads` to `FnMetadata::borrows`.
* Rename `FnMetadata::writes` to `FnMetadata::borrow_muts`.
* Rename `FnMetadataExt` to `IntoFnMetadata`.
* Add `IntoFnMeta`.
* Add `FnMeta` trait.
* Support 0 parameters.
* Support up to 8 parameters.

## 0.2.0 (2021-10-25)

* `FnMetadata::reads` returns `[TypeId; N]` instead of `Vec<TypeId>`.
* `FnMetadata::writes` returns `[TypeId; N]` instead of `Vec<TypeId>`.

## 0.1.0 (2021-04-26)

* Add `FnMetadataExt` to allow querying `TypeId`s of function parameters.