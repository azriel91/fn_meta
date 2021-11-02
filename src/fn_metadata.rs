use core::{any::TypeId, marker::PhantomData};

/// Metadata about a function
pub struct FnMetadata<Fun, Ret, Args>(pub(crate) PhantomData<(Fun, Ret, Args)>);

include!(concat!(env!("OUT_DIR"), "/lib_impl.rs"));
