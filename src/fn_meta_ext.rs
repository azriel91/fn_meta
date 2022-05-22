extern crate alloc;

use alloc::boxed::Box;

use crate::{FnMetaDyn, FnMetadata, FnMetadataExt};

/// Extension to return `Box<dyn FnMetaDyn>` for a function.
pub trait FnMetaExt<Fun, Ret, ArgRefs> {
    fn meta(&self) -> Box<dyn FnMetaDyn>;
}

impl<Fun, Ret> FnMetaExt<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret + FnMetadataExt<Fun, Ret, ()> + 'static,
    FnMetadata<Fun, Ret, ()>: FnMetaDyn,
    Ret: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A> FnMetaExt<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret + FnMetadataExt<Fun, Ret, (A,)> + 'static,
    FnMetadata<Fun, Ret, (A,)>: FnMetaDyn,
    Ret: 'static,
    A: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A, B> FnMetaExt<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret + FnMetadataExt<Fun, Ret, (A, B)> + 'static,
    FnMetadata<Fun, Ret, (A, B)>: FnMetaDyn,
    Ret: 'static,
    A: 'static,
    B: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A, B, C> FnMetaExt<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret + FnMetadataExt<Fun, Ret, (A, B, C)> + 'static,

    Ret: 'static,
    FnMetadata<Fun, Ret, (A, B, C)>: FnMetaDyn,
    A: 'static,
    B: 'static,
    C: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A, B, C, D> FnMetaExt<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret + FnMetadataExt<Fun, Ret, (A, B, C, D)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D)>: FnMetaDyn,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E> FnMetaExt<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret + FnMetadataExt<Fun, Ret, (A, B, C, D, E)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E)>: FnMetaDyn,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

impl<Fun, Ret, A, B, C, D, E, F> FnMetaExt<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret + FnMetadataExt<Fun, Ret, (A, B, C, D, E, F)> + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F)>: FnMetaDyn,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
{
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G> FnMetaExt<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret
        + FnMetadataExt<Fun, Ret, (A, B, C, D, E, F, G)>
        + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F, G)>: FnMetaDyn,
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
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G, H> FnMetaExt<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret
        + FnMetadataExt<Fun, Ret, (A, B, C, D, E, F, G, H)>
        + 'static,
    FnMetadata<Fun, Ret, (A, B, C, D, E, F, G, H)>: FnMetaDyn,
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
    fn meta(&self) -> Box<dyn FnMetaDyn> {
        Box::new(self.metadata())
    }
}
