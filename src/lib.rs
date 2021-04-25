#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::{any::TypeId, marker::PhantomData};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessMethod {
    Read,
    Write,
}

pub trait AccessMethodExt {
    fn access_method() -> AccessMethod;
    fn inner_type_id() -> TypeId;
}

impl<T> AccessMethodExt for &T
where
    T: 'static,
{
    fn access_method() -> AccessMethod {
        AccessMethod::Read
    }
    fn inner_type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

impl<T> AccessMethodExt for &mut T
where
    T: 'static,
{
    fn access_method() -> AccessMethod {
        AccessMethod::Write
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
    A: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B> FnMetadata<Fun, Ret, (A, B)>
where
    Fun: FnOnce(A, B) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C> FnMetadata<Fun, Ret, (A, B, C)>
where
    Fun: FnOnce(A, B, C) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
    C: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Read {
            type_ids.push(C::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Write {
            type_ids.push(C::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D> FnMetadata<Fun, Ret, (A, B, C, D)>
where
    Fun: FnOnce(A, B, C, D) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
    C: AccessMethodExt + 'static,
    D: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Read {
            type_ids.push(D::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Write {
            type_ids.push(D::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E> FnMetadata<Fun, Ret, (A, B, C, D, E)>
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
    C: AccessMethodExt + 'static,
    D: AccessMethodExt + 'static,
    E: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Read {
            type_ids.push(E::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Write {
            type_ids.push(E::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E, F> FnMetadata<Fun, Ret, (A, B, C, D, E, F)>
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
    C: AccessMethodExt + 'static,
    D: AccessMethodExt + 'static,
    E: AccessMethodExt + 'static,
    F: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Read {
            type_ids.push(E::inner_type_id());
        }
        if F::access_method() == AccessMethod::Read {
            type_ids.push(F::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Write {
            type_ids.push(E::inner_type_id());
        }
        if F::access_method() == AccessMethod::Write {
            type_ids.push(F::inner_type_id());
        }
        type_ids
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)>
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
    A: AccessMethodExt + 'static,
    B: AccessMethodExt + 'static,
    C: AccessMethodExt + 'static,
    D: AccessMethodExt + 'static,
    E: AccessMethodExt + 'static,
    F: AccessMethodExt + 'static,
    G: AccessMethodExt + 'static,
{
    pub fn reads(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Read {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Read {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Read {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Read {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Read {
            type_ids.push(E::inner_type_id());
        }
        if F::access_method() == AccessMethod::Read {
            type_ids.push(F::inner_type_id());
        }
        if G::access_method() == AccessMethod::Read {
            type_ids.push(G::inner_type_id());
        }
        type_ids
    }

    pub fn writes(&self) -> Vec<TypeId> {
        let mut type_ids = alloc::vec![];
        if A::access_method() == AccessMethod::Write {
            type_ids.push(A::inner_type_id());
        }
        if B::access_method() == AccessMethod::Write {
            type_ids.push(B::inner_type_id());
        }
        if C::access_method() == AccessMethod::Write {
            type_ids.push(C::inner_type_id());
        }
        if D::access_method() == AccessMethod::Write {
            type_ids.push(D::inner_type_id());
        }
        if E::access_method() == AccessMethod::Write {
            type_ids.push(E::inner_type_id());
        }
        if F::access_method() == AccessMethod::Write {
            type_ids.push(F::inner_type_id());
        }
        if G::access_method() == AccessMethod::Write {
            type_ids.push(G::inner_type_id());
        }
        type_ids
    }
}

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

    fn f_r1(_: &S0) -> () {}
    fn f_r2(_: &S0, _: &S1) -> () {}
    fn f_w1(_: &mut S0) -> () {}
    fn f_w2(_: &mut S0, _: &mut S1) -> () {}
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
