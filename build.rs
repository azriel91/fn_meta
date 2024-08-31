use std::{env, fmt::Write as _, fs::OpenOptions, io::Write, mem::MaybeUninit, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("Failed to read `OUT_DIR` environment variable.");
    let out_dir = Path::new(&out_dir);

    fn_metadata_impl(out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}

fn fn_metadata_impl(out_dir: &Path) {
    let fn_metadata_impl_path = out_dir.join("fn_metadata_impl.rs");
    let mut fn_metadata_impl = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(fn_metadata_impl_path)
        .expect("Failed to open `fn_metadata_impl.rs`.");

    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<1>())
        .expect("Failed to write to fn_metadata_impl.rs");
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<2>())
        .expect("Failed to write to fn_metadata_impl.rs");
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<3>())
        .expect("Failed to write to fn_metadata_impl.rs");
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<4>())
        .expect("Failed to write to fn_metadata_impl.rs");
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<5>())
        .expect("Failed to write to fn_metadata_impl.rs");
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<6>())
        .expect("Failed to write to fn_metadata_impl.rs");
    #[cfg(feature = "high_arg_count")]
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<7>())
        .expect("Failed to write to fn_metadata_impl.rs");
    #[cfg(feature = "high_arg_count")]
    write!(fn_metadata_impl, "{}", generate_impls_for_n_args::<8>())
        .expect("Failed to write to fn_metadata_impl.rs");

    fn_metadata_impl
        .flush()
        .expect("Failed to flush writer for fn_metadata_impl.rs");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ref {
    Immutable,
    Mutable,
}

fn generate_impls_for_n_args<const N: usize>() -> String {
    // "A0, A1"
    let args_csv = args_csv::<N>();

    // "    A0: 'static,\n    A1: 'static,"
    let arg_bounds_list = arg_bounds_list::<N>();

    #[cfg(feature = "fn_meta_ext")]
    let fn_meta_impl_buffer_len = 420 + N * 32;
    #[cfg(not(feature = "fn_meta_ext"))]
    let fn_meta_impl_buffer_len = 0;

    arg_refs_combinations::<N>().fold(
        String::with_capacity(N * (256 + N * 64 + fn_meta_impl_buffer_len)),
        |mut impls_buffer, arg_refs| {
            let mut arg_refs_iter = arg_refs.iter().copied().enumerate();

            // &mut A0, &A1
            let arg_refs_csv = {
                let mut arg_refs_csv = String::with_capacity(N * 8);
                if let Some((_index, arg_ref_first)) = arg_refs_iter.next() {
                    match arg_ref_first {
                        Ref::Immutable => arg_refs_csv.push_str("&A0"),
                        Ref::Mutable => arg_refs_csv.push_str("&mut A0"),
                    }
                }

                if N == 1 {
                    arg_refs_csv.push(',');
                } else {
                    arg_refs_iter
                        .try_for_each(|(index, arg_ref)| match arg_ref {
                            Ref::Immutable => write!(&mut arg_refs_csv, ", &A{index}"),
                            Ref::Mutable => write!(&mut arg_refs_csv, ", &mut A{index}"),
                        })
                        .expect("Failed to append to `arg_refs_csv` string.");
                }

                arg_refs_csv
            };

            // number of &ArgN
            let imm_refs_count = arg_refs
                .iter()
                .copied()
                .filter(|arg_ref| *arg_ref == Ref::Immutable)
                .count();

            // "TypeId::of::<A0>(), TypeId::of::<A1>()"
            let imm_ref_arg_ids = imm_ref_arg_ids::<N>(arg_refs);
            let mut_refs_count = arg_refs.len() - imm_refs_count;
            // "TypeId::of::<A0>(), TypeId::of::<A1>()"
            let mut_ref_arg_ids = mut_ref_arg_ids::<N>(arg_refs);

            write!(
                impls_buffer,
                r#"
impl<Fun, Ret, {args_csv}> FnMetadata<Fun, Ret, ({arg_refs_csv})>
where
{arg_bounds_list}
{{
    pub fn borrows() -> [TypeId; {imm_refs_count}] {{
        [{imm_ref_arg_ids}]
    }}

    pub fn borrow_muts() -> [TypeId; {mut_refs_count}] {{
        [{mut_ref_arg_ids}]
    }}
}}
"#,
            )
            .expect("Failed to append to impls_buffer.");

            #[cfg(feature = "fn_meta_ext")]
            write!(
                impls_buffer,
                r#"
impl<Fun, Ret, {args_csv}> crate::FnMeta for FnMetadata<Fun, Ret, ({arg_refs_csv})>
where
{arg_bounds_list}
{{
    fn borrows() -> crate::TypeIds {{
        let mut type_ids = crate::TypeIds::new();
        type_ids.extend(Self::borrows());
        type_ids
    }}

    fn borrow_muts() -> crate::TypeIds {{
        let mut type_ids = crate::TypeIds::new();
        type_ids.extend(Self::borrow_muts());
        type_ids
    }}
}}

impl<Fun, Ret, {args_csv}> crate::FnMetaDyn for FnMetadata<Fun, Ret, ({arg_refs_csv})>
where
{arg_bounds_list}
{{
    fn borrows(&self) -> crate::TypeIds {{
        let mut type_ids = crate::TypeIds::new();
        type_ids.extend(Self::borrows());
        type_ids
    }}

    fn borrow_muts(&self) -> crate::TypeIds {{
        let mut type_ids = crate::TypeIds::new();
        type_ids.extend(Self::borrow_muts());
        type_ids
    }}
}}
"#,
            )
            .expect("Failed to append to impls_buffer.");

            impls_buffer
        },
    )
}

fn mut_ref_arg_ids<const N: usize>(arg_refs: [Ref; N]) -> String {
    let mut mut_ref_arg_ids = String::with_capacity(N * 20);
    let mut arg_refs_mut_iter = arg_refs
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, arg_ref)| *arg_ref == Ref::Mutable);
    if let Some((index, _)) = arg_refs_mut_iter.next() {
        write!(&mut mut_ref_arg_ids, "TypeId::of::<A{index}>()")
            .expect("Failed to append to `mut_ref_arg_ids` string.")
    }
    arg_refs_mut_iter
        .try_for_each(|(index, _)| write!(&mut mut_ref_arg_ids, ", TypeId::of::<A{index}>()"))
        .expect("Failed to append to `mut_ref_arg_ids` string.");
    mut_ref_arg_ids
}

fn imm_ref_arg_ids<const N: usize>(arg_refs: [Ref; N]) -> String {
    let mut imm_ref_arg_ids = String::with_capacity(N * 20);
    let mut arg_refs_imm_iter = arg_refs
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, arg_ref)| *arg_ref == Ref::Immutable);
    if let Some((index, _)) = arg_refs_imm_iter.next() {
        write!(&mut imm_ref_arg_ids, "TypeId::of::<A{index}>()")
            .expect("Failed to append to `imm_ref_arg_ids` string.")
    }
    arg_refs_imm_iter
        .try_for_each(|(index, _)| write!(&mut imm_ref_arg_ids, ", TypeId::of::<A{index}>()"))
        .expect("Failed to append to `imm_ref_arg_ids` string.");
    imm_ref_arg_ids
}

fn arg_refs_combinations<const N: usize>() -> impl Iterator<Item = [Ref; N]> {
    (0..(2 << (N - 1))).map(|m| {
        // `m` is the combination variation count.
        // Whether an argument is immutable or mutable is based on its corresponding bit
        // value of `m`.

        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is safe
        // because the type we are claiming to have initialized here is a bunch of
        // `MaybeUninit`s, which do not require initialization.
        //
        // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        //
        // Switch this to `MaybeUninit::uninit_array` once it is stable.
        let mut arg_refs: [MaybeUninit<Ref>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        arg_refs
            .iter_mut()
            .enumerate()
            .for_each(move |(arg_n, arg_ref_mem)| {
                // for N = 5
                // m can be 0..32
                // if 31 >> 5 is 0

                if m >> arg_n & 1 == 0 {
                    arg_ref_mem.write(Ref::Immutable);
                } else {
                    arg_ref_mem.write(Ref::Mutable);
                }
            });

        // Everything is initialized. Transmute the array to the initialized type.
        // Unfortunately we cannot use this, see the following issues:
        //
        // * <https://github.com/rust-lang/rust/issues/61956>
        // * <https://github.com/rust-lang/rust/issues/80908>
        //
        // let arg_refs = unsafe { mem::transmute::<_, [Ref;
        // N]>(arg_refs) };

        #[allow(clippy::let_and_return)] // for clarity with `unsafe`
        let arg_refs = unsafe {
            (*(&MaybeUninit::new(arg_refs) as *const _ as *const MaybeUninit<_>)).assume_init_read()
        };

        arg_refs
    })
}

fn arg_bounds_list<const N: usize>() -> String {
    let mut arg_bounds_list = String::with_capacity(N * 17);
    arg_bounds_list.push_str("    A0: 'static,");
    (1..N).fold(arg_bounds_list, |mut arg_bounds_list, n| {
        write!(&mut arg_bounds_list, "\n    A{n}: 'static,")
            .expect("Failed to append to args_csv string.");
        arg_bounds_list
    })
}

fn args_csv<const N: usize>() -> String {
    let mut args_csv = String::with_capacity(N * 4);
    args_csv.push_str("A0");
    (1..N).fold(args_csv, |mut args_csv, n| {
        write!(&mut args_csv, ", A{n}").expect("Failed to append to args_csv string.");
        args_csv
    })
}
