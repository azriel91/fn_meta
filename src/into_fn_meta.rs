extern crate alloc;

use alloc::boxed::Box;

use crate::{FnMeta, FnMetadata, IntoFnMetadata};

/// Extension to return `Box<dyn FnMeta>` for a function.
pub trait IntoFnMeta<Fun, Ret, Args> {
    fn into_fn_meta(self) -> Box<dyn FnMeta>;
}

impl<Fun, Ret> IntoFnMeta<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret + IntoFnMetadata<Fun, Ret, ()> + 'static,
    FnMetadata<Fun, Ret, ()>: FnMeta,
    Ret: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A> IntoFnMeta<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret + IntoFnMetadata<Fun, Ret, (A,)> + 'static,
    FnMetadata<Fun, Ret, (A,)>: FnMeta,
    Ret: 'static,
    A: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B> IntoFnMeta<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret + IntoFnMetadata<Fun, Ret, (A, B)> + 'static,
    FnMetadata<Fun, Ret, (A, B)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C> IntoFnMeta<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret + IntoFnMetadata<Fun, Ret, (A, B, C)> + 'static,

    Ret: 'static,
    FnMetadata<Fun, Ret, (A, B, C)>: FnMeta,
    A: 'static,
    B: 'static,
    C: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnMeta<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret + IntoFnMetadata<Fun, Ret, (A, B, C, D)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnMeta<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret + IntoFnMetadata<Fun, Ret, (A, B, C, D, E)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnMeta<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret + IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
{
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnMeta<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret
        + IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F, G)>
        + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnMeta<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret
        + IntoFnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)>
        + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)>: FnMeta,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_meta(self) -> Box<dyn FnMeta> {
        Box::new(self.into_fn_metadata())
    }
}
