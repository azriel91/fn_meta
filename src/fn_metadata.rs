use core::{any::TypeId, marker::PhantomData};

/// Metadata about a function
pub struct FnMetadata<Fun, Ret, Args>(pub(crate) PhantomData<(Fun, Ret, Args)>);

impl<Fun, Ret> FnMetadata<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret,
{
    pub fn borrows(&self) -> [TypeId; 0] {
        []
    }

    pub fn borrow_muts(&self) -> [TypeId; 0] {
        []
    }
}

include!(concat!(env!("OUT_DIR"), "/lib_impl.rs"));
