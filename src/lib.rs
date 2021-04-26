#![no_std]

//! Returns metadata about a function.
//!
//! # Examples
//!
//! ```rust
//! # extern crate alloc;
//! #
//! # use core::any::TypeId;
//! #
//! use fn_meta::FnMetadataExt;
//!
//! fn my_function(_: &S0, _: &mut S1, _: &S2) -> () {}
//!
//! let fn_metadata = my_function.meta();
//!
//! assert_eq!(
//!     alloc::vec![TypeId::of::<S0>(), TypeId::of::<S2>()],
//!     fn_metadata.reads()
//! );
//! assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.writes());
//! #
//! # struct S0;
//! # struct S1;
//! # struct S2;
//! ```

extern crate alloc;

use alloc::vec::Vec;
use core::{any::TypeId, marker::PhantomData};

/// Whether the parameter is immutable or mutable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessKind {
    /// Read / shareable access.
    Read,
    /// Write / exclusive access.
    Write,
}

/// Extracts the [`AccessKind`] for a function parameter.
pub trait AccessKindExt {
    /// Returns the [`AccessKind`] of the parameter.
    fn access_kind() -> AccessKind;
    /// Returns the [`TypeId`] of the type being accessed.
    ///
    /// Notably, `T`, `&T`, and `&mut T` all have distinct `TypeId`s.
    ///
    /// ```rust
    /// # use core::any::TypeId;
    /// struct T;
    /// assert_ne!(TypeId::of::<T>(), TypeId::of::<&T>());
    /// assert_ne!(TypeId::of::<T>(), TypeId::of::<&mut T>());
    /// assert_ne!(TypeId::of::<&T>(), TypeId::of::<&mut T>());
    /// ```
    fn inner_type_id() -> TypeId;
}

impl<T> AccessKindExt for &T
where
    T: 'static,
{
    fn access_kind() -> AccessKind {
        AccessKind::Read
    }

    fn inner_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

impl<T> AccessKindExt for &mut T
where
    T: 'static,
{
    fn access_kind() -> AccessKind {
        AccessKind::Write
    }

    fn inner_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

/// Metadata about a function
pub struct FnMetadata<Fun, Ret, Args>(PhantomData<(Fun, Ret, Args)>);

impl<Fun, Ret, A> FnMetadata<Fun, Ret, (A,)>
where
    Fun: FnOnce(A) -> Ret,
    A: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B> FnMetadata<Fun, Ret, (A, B)>
where
    Fun: FnOnce(A, B) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C> FnMetadata<Fun, Ret, (A, B, C)>
where
    Fun: FnOnce(A, B, C) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
    C: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Read {
            type_ids.push(C::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Write {
            type_ids.push(C::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D> FnMetadata<Fun, Ret, (A, B, C, D)>
where
    Fun: FnOnce(A, B, C, D) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
    C: AccessKindExt + 'static,
    D: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Read {
            type_ids.push(D::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Write {
            type_ids.push(D::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E> FnMetadata<Fun, Ret, (A, B, C, D, E)>
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
    C: AccessKindExt + 'static,
    D: AccessKindExt + 'static,
    E: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Read {
            type_ids.push(E::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Write {
            type_ids.push(E::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E, F> FnMetadata<Fun, Ret, (A, B, C, D, E, F)>
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
    C: AccessKindExt + 'static,
    D: AccessKindExt + 'static,
    E: AccessKindExt + 'static,
    F: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Read {
            type_ids.push(E::inner_type_id());
        }
        if F::access_kind() == AccessKind::Read {
            type_ids.push(F::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Write {
            type_ids.push(E::inner_type_id());
        }
        if F::access_kind() == AccessKind::Write {
            type_ids.push(F::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)>
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
    A: AccessKindExt + 'static,
    B: AccessKindExt + 'static,
    C: AccessKindExt + 'static,
    D: AccessKindExt + 'static,
    E: AccessKindExt + 'static,
    F: AccessKindExt + 'static,
    G: AccessKindExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Read {
            type_ids.push(E::inner_type_id());
        }
        if F::access_kind() == AccessKind::Read {
            type_ids.push(F::inner_type_id());
        }
        if G::access_kind() == AccessKind::Read {
            type_ids.push(G::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_kind() == AccessKind::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_kind() == AccessKind::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_kind() == AccessKind::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_kind() == AccessKind::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_kind() == AccessKind::Write {
            type_ids.push(E::inner_type_id());
        }
        if F::access_kind() == AccessKind::Write {
            type_ids.push(F::inner_type_id());
        }
        if G::access_kind() == AccessKind::Write {
            type_ids.push(G::inner_type_id());
        }
        type_ids
    }
}

/// Extension to return [`FnMetadata`] for a function.
pub trait FnMetadataExt<Fun, Ret, Args> {
    fn meta(&self) -> FnMetadata<Fun, Ret, Args>;
}

impl<Fun, Ret, A> FnMetadataExt<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A,)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B> FnMetadataExt<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C> FnMetadataExt<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B, C)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D> FnMetadataExt<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B, C, D)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E> FnMetadataExt<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F> FnMetadataExt<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
{
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> FnMetadataExt<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn meta(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)> {
        FnMetadata(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use core::any::TypeId;

    use super::FnMetadataExt;

    #[test]
    fn read_1_write_1() {
        let fn_metadata = f_r1_w1.meta();
        assert_eq!(alloc::vec![TypeId::of::<S0>()], fn_metadata.reads());
        assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.writes());
    }

    #[test]
    fn write_1_read_1() {
        let fn_metadata = f_w1_r1.meta();
        assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.reads());
        assert_eq!(alloc::vec![TypeId::of::<S0>()], fn_metadata.writes());
    }

    #[test]
    fn read_1_write_1_read_1() {
        let fn_metadata = f_r1_w1_r1.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.reads()
        );
        assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.writes());
    }

    #[test]
    fn write_1_read_1_write_1() {
        let fn_metadata = f_w1_r1_w1.meta();
        assert_eq!(alloc::vec![TypeId::of::<S1>()], fn_metadata.reads());
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_2_read_2_write_2_read_1() {
        let fn_metadata = f_w2_r2_w2_r1.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            fn_metadata.reads()
        );
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_r1.meta();
        assert_eq!(alloc::vec![TypeId::of::<S0>()], fn_metadata.reads());
    }

    #[test]
    fn read_2() {
        let fn_metadata = f_r2.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_3() {
        let fn_metadata = f_r3.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.reads()
        );
    }

    #[test]
    fn read_4() {
        let fn_metadata = f_r4.meta();
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_r5.meta();
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_r6.meta();
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_w1.meta();
        assert_eq!(alloc::vec![TypeId::of::<S0>()], fn_metadata.writes());
    }

    #[test]
    fn write_2() {
        let fn_metadata = f_w2.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_3() {
        let fn_metadata = f_w3.meta();
        assert_eq!(
            alloc::vec![TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.writes()
        );
    }

    #[test]
    fn write_4() {
        let fn_metadata = f_w4.meta();
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_w5.meta();
        assert_eq!(
            alloc::vec![
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
        let fn_metadata = f_w6.meta();
        assert_eq!(
            alloc::vec![
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

    fn f_r1(_: &S0) -> () {}
    fn f_r2(_: &S0, _: &S1) -> () {}
    fn f_r3(_: &S0, _: &S1, _: &S2) -> () {}
    fn f_r4(_: &S0, _: &S1, _: &S2, _: &S3) -> () {}
    fn f_r5(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4) -> () {}
    fn f_r6(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5) -> () {}

    fn f_w1(_: &mut S0) -> () {}
    fn f_w2(_: &mut S0, _: &mut S1) -> () {}
    fn f_w3(_: &mut S0, _: &mut S1, _: &mut S2) -> () {}
    fn f_w4(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3) -> () {}
    fn f_w5(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4) -> () {}
    fn f_w6(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4, _: &mut S5) -> () {}

    fn f_r1_w1(_: &S0, _: &mut S1) -> () {}
    fn f_w1_r1(_: &mut S0, _: &S1) -> () {}
    fn f_r1_w1_r1(_: &S0, _: &mut S1, _: &S2) -> () {}
    fn f_w1_r1_w1(_: &mut S0, _: &S1, _: &mut S2) -> () {}
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
