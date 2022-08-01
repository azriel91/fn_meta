extern crate alloc;

use alloc::boxed::Box;

use crate::{FnMetadata, TypeIds};

/// Any type that tracks metadata about a function.
pub trait FnMeta {
    /// Returns the [`TypeId`]s of borrowed arguments.
    ///
    /// [`TypeId`]: core::any::TypeId
    fn borrows() -> TypeIds
    where
        Self: Sized;
    /// Returns the [`TypeId`]s of mutably borrowed arguments.
    ///
    /// [`TypeId`]: core::any::TypeId
    fn borrow_muts() -> TypeIds
    where
        Self: Sized;
}

impl<Fun, Ret> FnMeta for FnMetadata<Fun, Ret, ()> {
    fn borrows() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }

    fn borrow_muts() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl FnMeta for () {
    fn borrows() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }

    fn borrow_muts() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl FnMetaDyn for () {
    fn borrows(&self) -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }

    fn borrow_muts(&self) -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl<'any> FnMeta for &'any () {
    fn borrows() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }

    fn borrow_muts() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl<'any> FnMetaDyn for &'any () {
    fn borrows(&self) -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }

    fn borrow_muts(&self) -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl<T> FnMeta for Box<T>
where
    T: FnMeta,
{
    fn borrows() -> TypeIds {
        <T as FnMeta>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <T as FnMeta>::borrow_muts()
    }
}

impl<T> FnMeta for *mut T
where
    T: FnMeta,
{
    fn borrows() -> TypeIds {
        <T as FnMeta>::borrows()
    }

    fn borrow_muts() -> TypeIds {
        <T as FnMeta>::borrow_muts()
    }
}

/// Any type that tracks metadata about a function.
pub trait FnMetaDyn {
    /// Returns the [`TypeId`]s of borrowed arguments.
    ///
    /// [`TypeId`]: core::any::TypeId
    fn borrows(&self) -> TypeIds;
    /// Returns the [`TypeId`]s of mutably borrowed arguments.
    ///
    /// [`TypeId`]: core::any::TypeId
    fn borrow_muts(&self) -> TypeIds;
}

impl<Fun, Ret> FnMetaDyn for FnMetadata<Fun, Ret, ()> {
    fn borrows(&self) -> TypeIds {
        TypeIds::new()
    }

    fn borrow_muts(&self) -> TypeIds {
        TypeIds::new()
    }
}

impl<T> FnMetaDyn for Box<T>
where
    T: FnMetaDyn + ?Sized,
{
    fn borrows(&self) -> TypeIds {
        <T as FnMetaDyn>::borrows(self)
    }

    fn borrow_muts(&self) -> TypeIds {
        <T as FnMetaDyn>::borrow_muts(self)
    }
}

impl<T> FnMetaDyn for *mut T
where
    T: FnMetaDyn + ?Sized,
{
    fn borrows(&self) -> TypeIds {
        unsafe { (&**self).borrows() }
    }

    fn borrow_muts(&self) -> TypeIds {
        unsafe { (&**self).borrow_muts() }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::boxed::Box;
    use core::any::TypeId;

    use crate::{FnMeta, FnMetaDyn, FnMetaExt, FnMetadata};

    #[test]
    fn unit() {
        assert_eq!([] as [TypeId; 0], <() as FnMeta>::borrows().as_slice());
        assert_eq!([] as [TypeId; 0], <() as FnMeta>::borrow_muts().as_slice());
        assert_eq!([] as [TypeId; 0], ().borrows().as_slice());
        assert_eq!([] as [TypeId; 0], ().borrow_muts().as_slice());

        assert_eq!([] as [TypeId; 0], <&() as FnMeta>::borrows().as_slice());
        assert_eq!([] as [TypeId; 0], <&() as FnMeta>::borrow_muts().as_slice());
        assert_eq!([] as [TypeId; 0], FnMetaDyn::borrows(&&()).as_slice());
        assert_eq!([] as [TypeId; 0], FnMetaDyn::borrow_muts(&&()).as_slice());
    }

    #[test]
    fn no_args() {
        let fn_metadata = (|| {}).meta();
        assert_eq!(
            [] as [TypeId; 0],
            <FnMetadata<fn(), (), ()> as FnMeta>::borrows().as_slice()
        );
        assert_eq!(
            [] as [TypeId; 0],
            <FnMetadata<fn(), (), ()> as FnMeta>::borrow_muts().as_slice()
        );
        assert_eq!([] as [TypeId; 0], fn_metadata.borrows().as_slice());
        assert_eq!([] as [TypeId; 0], fn_metadata.borrow_muts().as_slice());
    }

    #[test]
    fn box_fn_meta() {
        assert_eq!(
            [] as [TypeId; 0],
            <Box<FnMetadata<fn(), (), ()>> as FnMeta>::borrows().as_slice()
        );
        assert_eq!(
            [] as [TypeId; 0],
            <Box<FnMetadata<fn(), (), ()>> as FnMeta>::borrow_muts().as_slice()
        );
    }

    #[test]
    fn ptr_fn_meta() {
        assert_eq!(
            [] as [TypeId; 0],
            <*mut FnMetadata<fn(), (), ()> as FnMeta>::borrows().as_slice()
        );
        assert_eq!(
            [] as [TypeId; 0],
            <*mut FnMetadata<fn(), (), ()> as FnMeta>::borrow_muts().as_slice()
        );

        let ptr: *mut () = &mut ();
        assert_eq!(
            [] as [TypeId; 0],
            <*mut () as FnMetaDyn>::borrows(&ptr).as_slice()
        );
        assert_eq!(
            [] as [TypeId; 0],
            <*mut () as FnMetaDyn>::borrow_muts(&ptr).as_slice()
        );
    }

    #[test]
    fn read_1_write_1() {
        let fn_metadata = f_r1_w1.meta();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrows().as_slice());
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts().as_slice());
    }

    #[test]
    fn write_1_read_1() {
        let fn_metadata = f_w1_r1.meta();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrows().as_slice());
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrow_muts().as_slice());
    }

    #[test]
    fn read_1_write_1_read_1() {
        let fn_metadata = f_r1_w1_r1.meta();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.borrows().as_slice()
        );
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrow_muts().as_slice());
    }

    #[test]
    fn write_1_read_1_write_1() {
        let fn_metadata = f_w1_r1_w1.meta();
        assert_eq!([TypeId::of::<S1>()], fn_metadata.borrows().as_slice());
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S2>()],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn write_2_read_2_write_2_read_1() {
        let fn_metadata = f_w2_r2_w2_r1.meta();
        assert_eq!(
            [TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            fn_metadata.borrows().as_slice()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn write_2_read_2_write_2_read_2() {
        let fn_metadata =
            (|_: &mut S0, _: &mut S1, _: &S2, _: &S3, _: &mut S4, _: &mut S5, _: &S6, _: &S7| {})
                .meta();
        assert_eq!(
            [
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S6>(),
                TypeId::of::<S7>()
            ],
            fn_metadata.borrows().as_slice()
        );
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn read_1() {
        let fn_metadata = f_r1.meta();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrows().as_slice());
    }

    #[test]
    fn read_2() {
        let fn_metadata = f_r2.meta();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.borrows().as_slice()
        );
    }

    #[test]
    fn read_3() {
        let fn_metadata = f_r3.meta();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.borrows().as_slice()
        );
    }

    #[test]
    fn read_4() {
        let fn_metadata = f_r4.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.borrows().as_slice()
        );
    }

    #[test]
    fn read_5() {
        let fn_metadata = f_r5.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.borrows().as_slice()
        );
    }

    #[test]
    fn read_6() {
        let fn_metadata = f_r6.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrows().as_slice()
        );
    }

    #[test]
    fn write_1() {
        let fn_metadata = f_w1.meta();
        assert_eq!([TypeId::of::<S0>()], fn_metadata.borrow_muts().as_slice());
    }

    #[test]
    fn write_2() {
        let fn_metadata = f_w2.meta();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>()],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn write_3() {
        let fn_metadata = f_w3.meta();
        assert_eq!(
            [TypeId::of::<S0>(), TypeId::of::<S1>(), TypeId::of::<S2>(),],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn write_4() {
        let fn_metadata = f_w4.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
            ],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn write_5() {
        let fn_metadata = f_w5.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
            ],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn write_6() {
        let fn_metadata = f_w6.meta();
        assert_eq!(
            [
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S2>(),
                TypeId::of::<S3>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            fn_metadata.borrow_muts().as_slice()
        );
    }

    #[test]
    fn coverage_completeness() {
        f_r1(&S0);
        f_r2(&S0, &S1);
        f_r3(&S0, &S1, &S2);
        f_r4(&S0, &S1, &S2, &S3);
        f_r5(&S0, &S1, &S2, &S3, &S4);
        f_r6(&S0, &S1, &S2, &S3, &S4, &S5);

        f_w1(&mut S0);
        f_w2(&mut S0, &mut S1);
        f_w3(&mut S0, &mut S1, &mut S2);
        f_w4(&mut S0, &mut S1, &mut S2, &mut S3);
        f_w5(&mut S0, &mut S1, &mut S2, &mut S3, &mut S4);
        f_w6(&mut S0, &mut S1, &mut S2, &mut S3, &mut S4, &mut S5);

        f_r1_w1(&S0, &mut S1);
        f_w1_r1(&mut S0, &S1);
        f_r1_w1_r1(&S0, &mut S1, &S2);
        f_w1_r1_w1(&mut S0, &S1, &mut S2);
        #[cfg(feature = "high_arg_count")]
        f_w2_r2_w2_r1(&mut S0, &mut S1, &S2, &S3, &mut S4, &mut S5, &S6);
    }

    fn f_r1(_: &S0) {}
    fn f_r2(_: &S0, _: &S1) {}
    fn f_r3(_: &S0, _: &S1, _: &S2) {}
    fn f_r4(_: &S0, _: &S1, _: &S2, _: &S3) {}
    fn f_r5(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4) {}
    fn f_r6(_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5) {}

    fn f_w1(_: &mut S0) {}
    fn f_w2(_: &mut S0, _: &mut S1) {}
    fn f_w3(_: &mut S0, _: &mut S1, _: &mut S2) {}
    fn f_w4(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3) {}
    fn f_w5(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4) {}
    fn f_w6(_: &mut S0, _: &mut S1, _: &mut S2, _: &mut S3, _: &mut S4, _: &mut S5) {}

    fn f_r1_w1(_: &S0, _: &mut S1) {}
    fn f_w1_r1(_: &mut S0, _: &S1) {}
    fn f_r1_w1_r1(_: &S0, _: &mut S1, _: &S2) {}
    fn f_w1_r1_w1(_: &mut S0, _: &S1, _: &mut S2) {}
    #[cfg(feature = "high_arg_count")]
    fn f_w2_r2_w2_r1(_: &mut S0, _: &mut S1, _: &S2, _: &S3, _: &mut S4, _: &mut S5, _: &S6) {}

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
