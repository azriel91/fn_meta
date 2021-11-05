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

include!(concat!(env!("OUT_DIR"), "/fn_metadata_impl.rs"));

#[cfg(test)]
mod tests {
    use core::any::TypeId;

    use crate::FnMetadataExt;

    #[test]
    fn no_args() {
        let fn_metadata = (|| {}).metadata();
        assert_eq!([] as [TypeId; 0], fn_metadata.borrows());
        assert_eq!([] as [TypeId; 0], fn_metadata.borrow_muts());
    }

    #[test]
    fn read_1_write_1() {
        let fn_metadata = f_r1_w1.metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrows());
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts());
    }

    #[test]
    fn write_1_read_1() {
        let fn_metadata = f_w1_r1.metadata();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrows());
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrow_muts());
    }

    #[test]
    fn read_1_write_1_read_1() {
        let fn_metadata = f_r1_w1_r1.metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.borrows()
        );
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts());
    }

    #[test]
    fn write_1_read_1_write_1() {
        let fn_metadata = f_w1_r1_w1.metadata();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrows());
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_2_read_2_write_2_read_1() {
        let fn_metadata = f_w2_r2_w2_r1.metadata();
        assert_eq!(
            [TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            fn_metadata.borrows()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_2_read_2_write_2_read_2() {
        let fn_metadata =
            (|_: &mut S0, _: &mut S1, _: &S2, _: &S3, _: &mut S4, _: &mut S5, _: &S6, _: &S7| {})
                .metadata();
        assert_eq!(
            [
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S6>(),
                TypeId::of::<S7>()
            ],
            fn_metadata.borrows()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn read_1() {
        let fn_metadata = f_r1.metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrows());
    }

    #[test]
    fn read_2() {
        let fn_metadata = f_r2.metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.borrows()
        );
    }

    #[test]
    fn read_3() {
        let fn_metadata = f_r3.metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.borrows()
        );
    }

    #[test]
    fn read_4() {
        let fn_metadata = f_r4.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.borrows()
        );
    }

    #[test]
    fn read_5() {
        let fn_metadata = f_r5.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.borrows()
        );
    }

    #[test]
    fn read_6() {
        let fn_metadata = f_r6.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrows()
        );
    }

    #[test]
    fn write_1() {
        let fn_metadata = f_w1.metadata();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrow_muts());
    }

    #[test]
    fn write_2() {
        let fn_metadata = f_w2.metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_3() {
        let fn_metadata = f_w3.metadata();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_4() {
        let fn_metadata = f_w4.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_5() {
        let fn_metadata = f_w5.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.borrow_muts()
        );
    }

    #[test]
    fn write_6() {
        let fn_metadata = f_w6.metadata();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts()
        );
    }

    #[cfg(not(tarpaulin_include))]
    fn f_r1(_: &S0) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r2(_: &S0, _: &S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r3(_: &S0, _: &S1, _: &S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r4(_: &S0, _: &S1, _: &S2, _: &S3) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r5(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r6(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5) -> () {}

    #[cfg(not(tarpaulin_include))]
    fn f_w1(_: &mut S0) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w2(_: &mut S0, _: &mut S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w3(_: &mut S0, _: &mut S1, _: &mut S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w4(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w5(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w6(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4, _: &mut S5) -> () {}

    #[cfg(not(tarpaulin_include))]
    fn f_r1_w1(_: &S0, _: &mut S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w1_r1(_: &mut S0, _: &S1) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_r1_w1_r1(_: &S0, _: &mut S1, _: &S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w1_r1_w1(_: &mut S0, _: &S1, _: &mut S2) -> () {}
    #[cfg(not(tarpaulin_include))]
    fn f_w2_r2_w2_r1(_: &mut S0, _: &mut S1, _: &S2, _: &S3, _: &mut S4, _: &mut S5, _: &S6) -> () {
    }

    struct S0;
    struct S1;
    struct S2;
    struct S3;
    struct S4;
    struct S5;
    struct S6;
    struct S7;
}
