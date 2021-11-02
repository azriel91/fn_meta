use core::marker::PhantomData;

use crate::FnMetadata;

/// Extension to return [`FnMetadata`] for a function.
pub trait IntoFnMetadata<Fun, Ret, Args> {
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, Args>;
}

impl<Fun, Ret> IntoFnMetadata<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, ()> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A> IntoFnMetadata<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A,)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B> IntoFnMetadata<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C> IntoFnMetadata<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnMetadata<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C, D)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnMetadata<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C, D, E)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
{
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)> {
        FnMetadata(PhantomData)
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_metadata(self) -> FnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)> {
        FnMetadata(PhantomData)
    }
}
