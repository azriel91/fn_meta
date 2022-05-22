use core::{any::TypeId, marker::PhantomData};

/// Metadata about a function
pub struct FnMetadata<Fun, Ret, ArgRefs>(pub PhantomData<(Fun, Ret, ArgRefs)>);

impl<Fun, Ret> FnMetadata<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret,
{
    pub fn borrows() -> [TypeId; 0] {
        []
    }

    pub fn borrow_muts() -> [TypeId; 0] {
        []
    }
}

include!(concat!(env!("OUT_DIR"), "/fn_metadata_impl.rs"));

#[cfg(test)]
mod tests {
    use core::any::TypeId;

    use crate::FnMetadata;

    #[test]
    fn no_args() {
        assert_eq!([] as [TypeId; 0], FnMetadata::<fn(), (), ()>::borrows());
        assert_eq!([] as [TypeId; 0], FnMetadata::<fn(), (), ()>::borrow_muts());
    }

    #[test]
    fn read_1_write_1() {
        assert_eq!(
            [TypeId::of::<S0>()],
            FnMetadata::<fn(), (), (&S0, &mut S1)>::borrows()
        );
        assert_eq!(
            [TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&S0, &mut S1)>::borrow_muts()
        );
    }

    #[test]
    fn write_1_read_1() {
        assert_eq!(
            [TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&mut S0, &S1)>::borrows()
        );
        assert_eq!(
            [TypeId::of::<S0>()],
            FnMetadata::<fn(), (), (&mut S0, &S1)>::borrow_muts()
        );
    }

    #[test]
    fn read_1_write_1_read_1() {
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            FnMetadata::<fn(), (), (&S0, &mut S1, &S2)>::borrows()
        );
        assert_eq!(
            [TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&S0, &mut S1, &S2)>::borrow_muts()
        );
    }

    #[test]
    fn write_1_read_1_write_1() {
        assert_eq!(
            [TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&mut S0, &S1, &mut S2)>::borrows()
        );
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            FnMetadata::<fn(), (), (&mut S0, &S1, &mut S2)>::borrow_muts()
        );
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn write_2_read_2_write_2_read_1() {
        assert_eq!(
            [TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &S2, &S3, &mut S4, &mut S5, &S6)>::borrows()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &S2, &S3, &mut S4, &mut S5, &S6)>::borrow_muts()
        );
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn write_2_read_2_write_2_read_2() {
        assert_eq!(
            [
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S6>(),
                TypeId::of::<S7>()
            ],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &S2, &S3, &mut S4, &mut S5, &S6, &S7)>::borrows()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &S2, &S3, &mut S4, &mut S5, &S6, &S7)>::borrow_muts()
        );
    }

    #[test]
    fn read_1() {
        assert_eq!(
            [TypeId::of::<S0>()],
            FnMetadata::<fn(), (), (&S0,)>::borrows()
        );
    }

    #[test]
    fn read_2() {
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&S0, &S1)>::borrows()
        );
    }

    #[test]
    fn read_3() {
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            FnMetadata::<fn(), (), (&S0, &S1, &S2)>::borrows()
        );
    }

    #[test]
    fn read_4() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            FnMetadata::<fn(), (), (&S0, &S1, &S2, &S3)>::borrows()
        );
    }

    #[test]
    fn read_5() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            FnMetadata::<fn(), (), (&S0, &S1, &S2, &S3, &S4)>::borrows()
        );
    }

    #[test]
    fn read_6() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            FnMetadata::<fn(), (), (&S0, &S1, &S2, &S3, &S4, &S5)>::borrows()
        );
    }

    #[test]
    fn write_1() {
        assert_eq!(
            [TypeId::of::<S0>()],
            FnMetadata::<fn(), (), (&mut S0,)>::borrow_muts()
        );
    }

    #[test]
    fn write_2() {
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            FnMetadata::<fn(), (), (&mut S0, &mut S1)>::borrow_muts()
        );
    }

    #[test]
    fn write_3() {
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &mut S2)>::borrow_muts()
        );
    }

    #[test]
    fn write_4() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &mut S2, &mut S3)>::borrow_muts()
        );
    }

    #[test]
    fn write_5() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            FnMetadata::<fn(), (), (&mut S0, &mut S1, &mut S2, &mut S3, &mut S4)>::borrow_muts()
        );
    }

    #[test]
    fn write_6() {
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            FnMetadata::<fn(), (), (&mut S0,&mut S1,&mut S2,&mut S3,&mut S4,&mut S5)>::borrow_muts()
        );
    }

    struct S0;
    struct S1;
    struct S2;
    struct S3;
    struct S4;
    struct S5;
    #[cfg(feature = "high_arg_count")]
    struct S6;
    #[cfg(feature = "high_arg_count")]
    struct S7;
}
