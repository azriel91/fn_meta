use core::marker::PhantomData;

use crate::FnMetadata;

/// Extension to return [`FnMetadata`] for a function.
pub trait FnMetadataExt<Fun, Ret, ArgRefs> {
    fn metadata(&self) -> FnMetadata<Fun, Ret, ArgRefs>;
}

impl<Fun, Ret> FnMetadataExt<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, ()> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A> FnMetadataExt<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A,)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B> FnMetadataExt<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C> FnMetadataExt<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D> FnMetadataExt<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C, D)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E> FnMetadataExt<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F> FnMetadataExt<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
{
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F)> {
        FnMetadata(PhantomData)
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G> FnMetadataExt<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)> {
        FnMetadata(PhantomData)
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G, H> FnMetadataExt<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn metadata(&self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)> {
        FnMetadata(PhantomData)
    }
}
