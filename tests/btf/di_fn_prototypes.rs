#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

trait Test {
    fn modify(&mut self);
}

impl<T> Test for Option<T>
where
    T: Test + Copy,
{
    fn modify(&mut self) {
        match *self {
            Some(mut t) => t.modify(),
            _ => {}
        }
    }
}

impl Test for bool {
    fn modify(&mut self) {
        *self ^= *self;
    }
}

macro_rules! impl_test_for_num {
    ($($type:ty),*) => {
        $(impl Test for $type {
            fn modify(&mut self) {
                *self += *self;
            }
        })*
    };
}

impl_test_for_num!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl Test for &mut str {
    fn modify(&mut self) {
        let mut bytes = unsafe { self.as_bytes_mut() };
        if !bytes.is_empty() {
            let b = bytes[0] as u8;
            bytes[0] *= bytes[0];
        }
    }
}

macro_rules! impl_test_for_slice_num {
    ($($type:ty),*) => {
        $(impl Test for $type {
            fn modify(&mut self) {
                self.iter_mut().for_each(|i| *i += *i);
            }
        })*
    };
}

impl_test_for_slice_num!(
    &mut [i8],
    &mut [i16],
    &mut [i32],
    &mut [i64],
    &mut [i128],
    &mut [isize],
    &mut [u8],
    &mut [u16],
    &mut [u32],
    &mut [u64],
    &mut [u128],
    &mut [usize],
    &mut [f32],
    &mut [f64],
    [i8; 3],
    [i16; 3],
    [i32; 3],
    [i64; 3],
    [i128; 3],
    [isize; 3],
    [u8; 3],
    [u16; 3],
    [u32; 3],
    [u64; 3],
    [u128; 3],
    [usize; 3],
    [f32; 3],
    [f64; 3]
);

#[repr(C)]
#[derive(Clone, Copy)]
enum TestEnum<T> {
    A,
    B(T),
    C(T, T),
    D { x: T, y: T },
}

#[repr(C)]
union TestUnion<T>
where
    T: Copy,
{
    a: T,
    b: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TestStruct<T>
where
    T: Copy,
{
    x: T,
    e: TestEnum<T>,
    o: Option<T>,
    //u: TestUnion<T>,
}

impl<T> TestStruct<T>
where
    T: Copy,
{
    const fn new(t: T) -> Self {
        Self {
            x: t,
            e: TestEnum::B(t),
            o: Some(t),
            //u: TestUnion { a: t },
        }
    }
}

impl<T> Test for TestStruct<T>
where
    T: Test + Copy,
{
    fn modify(&mut self) {
        self.x.modify()
    }
}

impl<T> Test for TestEnum<T>
where
    T: Test + Copy,
{
    fn modify(&mut self) {
        match *self {
            Self::B(mut b) => {
                b.modify();
                *self = Self::B(b);
            }
            Self::C(mut c1, mut c2) => {
                c1.modify();
                c2.modify();
                *self = Self::C(c1, c2);
            }
            Self::D { mut x, mut y } => {
                x.modify();
                y.modify();
                *self = Self::D { x, y };
            }
            _ => {}
        }
    }
}

#[inline(never)]
fn test_args_1<T: Test>(mut t: T) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

#[inline(never)]
fn test_args_2<T: Test, U: Test>(mut t: T, mut u: U) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

#[inline(never)]
fn test_args_3<T: Test, U: Test, V: Test>(mut t: T, mut u: U, mut v: V) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

#[inline(never)]
fn test_args_4<T: Test, U: Test, V: Test, W: Test>(mut t: T, mut u: U, mut v: V, mut w: W) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

#[inline(never)]
fn test_args_5<T: Test, U: Test, V: Test, W: Test, X: Test>(
    mut t: T,
    mut u: U,
    mut v: V,
    mut w: W,
    mut x: X,
) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

#[inline(never)]
fn test_args_6<T: Test, U: Test, V: Test, W: Test, X: Test, Y: Test>(
    mut t: T,
    mut u: U,
    mut v: V,
    mut w: W,
    mut x: X,
    mut Y: Y,
) {
    // to minimize chances to get inlined
    (0..64).for_each(|_| t.modify());
}

static mut ARG_BUF_I8: [i8; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_I16: [i16; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_I32: [i32; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_I64: [i64; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_I128: [i128; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_ISIZE: [isize; 3] = [0x41, 0x79, 0x61];

static mut ARG_BUF_U8: [u8; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_U16: [u16; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_U32: [u32; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_U64: [u64; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_U128: [u128; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_USIZE: [usize; 3] = [0x41, 0x79, 0x61];
static mut ARG_BUF_F32: [f32; 3] = [65.0, 121.0, 97.0];
static mut ARG_BUF_F64: [f64; 3] = [65.0, 121.0, 97.0];

static mut ARG_OPT_BUF_I8: Option<[i8; 3]> = Some(unsafe { ARG_BUF_I8 });
static mut ARG_OPT_BUF_I16: Option<[i16; 3]> = Some(unsafe { ARG_BUF_I16 });
static mut ARG_OPT_BUF_I32: Option<[i32; 3]> = Some(unsafe { ARG_BUF_I32 });
static mut ARG_OPT_BUF_I64: Option<[i64; 3]> = Some(unsafe { ARG_BUF_I64 });
static mut ARG_OPT_BUF_ISIZE: Option<[isize; 3]> = Some(unsafe { ARG_BUF_ISIZE });
static mut ARG_OPT_BUF_U8: Option<[u8; 3]> = Some(unsafe { ARG_BUF_U8 });
static mut ARG_OPT_BUF_U16: Option<[u16; 3]> = Some(unsafe { ARG_BUF_U16 });
static mut ARG_OPT_BUF_U32: Option<[u32; 3]> = Some(unsafe { ARG_BUF_U32 });
static mut ARG_OPT_BUF_U64: Option<[u64; 3]> = Some(unsafe { ARG_BUF_U64 });
static mut ARG_OPT_BUF_USIZE: Option<[usize; 3]> = Some(unsafe { ARG_BUF_USIZE });
static mut ARG_OPT_BUF_F32: Option<[f32; 3]> = Some(unsafe { ARG_BUF_F32 });
static mut ARG_OPT_BUF_F64: Option<[f64; 3]> = Some(unsafe { ARG_BUF_F64 });
static mut ARG_OPT_I8: Option<i8> = Some(42i8);
static mut ARG_OPT_I16: Option<i16> = Some(42i16);
static mut ARG_OPT_I32: Option<i32> = Some(42i32);
static mut ARG_OPT_I64: Option<i64> = Some(42i64);
static mut ARG_OPT_ISIZE: Option<isize> = Some(42isize);
static mut ARG_OPT_U8: Option<u8> = Some(42u8);
static mut ARG_OPT_U16: Option<u16> = Some(42u16);
static mut ARG_OPT_U32: Option<u32> = Some(42u32);
static mut ARG_OPT_U64: Option<u64> = Some(42u64);
static mut ARG_OPT_USIZE: Option<usize> = Some(42usize);
static mut ARG_OPT_F32: Option<f32> = Some(42.0f32);
static mut ARG_OPT_F64: Option<f64> = Some(42.0f64);

static mut ARG_STRUCT_BUF_I8: TestStruct<[i8; 3]> = TestStruct::new(unsafe { ARG_BUF_I8 });
static mut ARG_STRUCT_BUF_I16: TestStruct<[i16; 3]> = TestStruct::new(unsafe { ARG_BUF_I16 });
static mut ARG_STRUCT_BUF_I32: TestStruct<[i32; 3]> = TestStruct::new(unsafe { ARG_BUF_I32 });
static mut ARG_STRUCT_BUF_I64: TestStruct<[i64; 3]> = TestStruct::new(unsafe { ARG_BUF_I64 });
static mut ARG_STRUCT_BUF_I128: TestStruct<[i128; 3]> = TestStruct::new(unsafe { ARG_BUF_I128 });
static mut ARG_STRUCT_BUF_ISIZE: TestStruct<[isize; 3]> = TestStruct::new(unsafe { ARG_BUF_ISIZE });
static mut ARG_STRUCT_BUF_U8: TestStruct<[u8; 3]> = TestStruct::new(unsafe { ARG_BUF_U8 });
static mut ARG_STRUCT_BUF_U16: TestStruct<[u16; 3]> = TestStruct::new(unsafe { ARG_BUF_U16 });
static mut ARG_STRUCT_BUF_U32: TestStruct<[u32; 3]> = TestStruct::new(unsafe { ARG_BUF_U32 });
static mut ARG_STRUCT_BUF_U64: TestStruct<[u64; 3]> = TestStruct::new(unsafe { ARG_BUF_U64 });
static mut ARG_STRUCT_BUF_U128: TestStruct<[u128; 3]> = TestStruct::new(unsafe { ARG_BUF_U128 });
static mut ARG_STRUCT_BUF_USIZE: TestStruct<[usize; 3]> = TestStruct::new(unsafe { ARG_BUF_USIZE });
static mut ARG_STRUCT_BUF_F32: TestStruct<[f32; 3]> = TestStruct::new(unsafe { ARG_BUF_F32 });
static mut ARG_STRUCT_BUF_F64: TestStruct<[f64; 3]> = TestStruct::new(unsafe { ARG_BUF_F64 });
static mut ARG_STRUCT_I8: TestStruct<i8> = TestStruct::new(42i8);
static mut ARG_STRUCT_I16: TestStruct<i16> = TestStruct::new(42i16);
static mut ARG_STRUCT_I32: TestStruct<i32> = TestStruct::new(42i32);
static mut ARG_STRUCT_I64: TestStruct<i64> = TestStruct::new(42i64);
static mut ARG_STRUCT_I128: TestStruct<i128> = TestStruct::new(42i128);
static mut ARG_STRUCT_ISIZE: TestStruct<isize> = TestStruct::new(42isize);
static mut ARG_STRUCT_U8: TestStruct<u8> = TestStruct::new(42u8);
static mut ARG_STRUCT_U16: TestStruct<u16> = TestStruct::new(42u16);
static mut ARG_STRUCT_U32: TestStruct<u32> = TestStruct::new(42u32);
static mut ARG_STRUCT_U64: TestStruct<u64> = TestStruct::new(42u64);
static mut ARG_STRUCT_U128: TestStruct<u128> = TestStruct::new(42u128);
static mut ARG_STRUCT_USIZE: TestStruct<usize> = TestStruct::new(42usize);
static mut ARG_STRUCT_F32: TestStruct<f32> = TestStruct::new(42.0f32);
static mut ARG_STRUCT_F64: TestStruct<f64> = TestStruct::new(42.0f64);

static mut ARG_ENUM_BUF_I8: TestEnum<TestEnum<TestEnum<[i8; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_I8 })),
};
static mut ARG_ENUM_BUF_I16: TestEnum<TestEnum<TestEnum<[i16; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_I16 })),
};
static mut ARG_ENUM_BUF_I32: TestEnum<TestEnum<TestEnum<[i32; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_I32 })),
};
static mut ARG_ENUM_BUF_I64: TestEnum<TestEnum<TestEnum<[i64; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_I64 })),
};
static mut ARG_ENUM_BUF_ISIZE: TestEnum<TestEnum<TestEnum<[isize; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_ISIZE })),
};
static mut ARG_ENUM_BUF_U8: TestEnum<TestEnum<TestEnum<[u8; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_U8 })),
};
static mut ARG_ENUM_BUF_U16: TestEnum<TestEnum<TestEnum<[u16; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_U16 })),
};
static mut ARG_ENUM_BUF_U32: TestEnum<TestEnum<TestEnum<[u32; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_U32 })),
};
static mut ARG_ENUM_BUF_U64: TestEnum<TestEnum<TestEnum<[u64; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_U64 })),
};
static mut ARG_ENUM_BUF_USIZE: TestEnum<TestEnum<TestEnum<[usize; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_USIZE })),
};
static mut ARG_ENUM_BUF_F32: TestEnum<TestEnum<TestEnum<[f32; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_F32 })),
};
static mut ARG_ENUM_BUF_F64: TestEnum<TestEnum<TestEnum<[f64; 3]>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(unsafe { ARG_BUF_F64 })),
};
static mut ARG_ENUM_I8: TestEnum<TestEnum<TestEnum<i8>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42i8)),
};
static mut ARG_ENUM_I16: TestEnum<TestEnum<TestEnum<i16>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42i16)),
};
static mut ARG_ENUM_I32: TestEnum<TestEnum<TestEnum<i32>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42i32)),
};
static mut ARG_ENUM_I64: TestEnum<TestEnum<TestEnum<i64>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42i64)),
};
static mut ARG_ENUM_ISIZE: TestEnum<TestEnum<TestEnum<isize>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42isize)),
};
static mut ARG_ENUM_U8: TestEnum<TestEnum<TestEnum<u8>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42u8)),
};
static mut ARG_ENUM_U16: TestEnum<TestEnum<TestEnum<u16>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42u16)),
};
static mut ARG_ENUM_U32: TestEnum<TestEnum<TestEnum<u32>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42u32)),
};
static mut ARG_ENUM_U64: TestEnum<TestEnum<TestEnum<u64>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42u64)),
};
static mut ARG_ENUM_USIZE: TestEnum<TestEnum<TestEnum<usize>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42usize)),
};
static mut ARG_ENUM_F32: TestEnum<TestEnum<TestEnum<f32>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42.0f32)),
};
static mut ARG_ENUM_F64: TestEnum<TestEnum<TestEnum<f64>>> = TestEnum::D {
    x: TestEnum::A,
    y: TestEnum::C(TestEnum::A, TestEnum::B(42.0f64)),
};

macro_rules! arg_str {
    () => {
        core::str::from_utf8_unchecked_mut(ARG_BUF_U8.as_mut_slice())
    };
}
#[no_mangle]
#[link_section = "uprobe/test_1"]
unsafe fn test_1() {
    test_args_1(ARG_BUF_I8.as_mut_slice());
}
// CHECK: FUNC 'test_1' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i8_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_2"]
unsafe fn test_2() {
    test_args_1(ARG_BUF_I16.as_mut_slice());
}
// CHECK: FUNC 'test_2' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i16_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_3"]
unsafe fn test_3() {
    test_args_1(ARG_BUF_I32.as_mut_slice());
}
// CHECK: FUNC 'test_3' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_4"]
unsafe fn test_4() {
    test_args_1(ARG_BUF_I64.as_mut_slice());
}
// CHECK: FUNC 'test_4' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i64_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_5"]
unsafe fn test_5() {
    test_args_1(ARG_BUF_ISIZE.as_mut_slice());
}
// CHECK: FUNC 'test_5' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_isize_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_6"]
unsafe fn test_6() {
    test_args_1(ARG_BUF_U8.as_mut_slice());
}
// CHECK: FUNC 'test_6' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u8_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_7"]
unsafe fn test_7() {
    test_args_1(ARG_BUF_U16.as_mut_slice());
}
// CHECK: FUNC 'test_7' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u16_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_8"]
unsafe fn test_8() {
    test_args_1(ARG_BUF_U32.as_mut_slice());
}
// CHECK: FUNC 'test_8' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u32_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_9"]
unsafe fn test_9() {
    test_args_1(ARG_BUF_U64.as_mut_slice());
}
// CHECK: FUNC 'test_9' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u64_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_10"]
unsafe fn test_10() {
    test_args_1(ARG_BUF_USIZE.as_mut_slice());
}
// CHECK: FUNC 'test_10' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_usize_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_11"]
unsafe fn test_11() {
    test_args_1(ARG_BUF_F32.as_mut_slice());
}
// CHECK: FUNC 'test_11' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_12"]
unsafe fn test_12() {
    test_args_1(ARG_BUF_F64.as_mut_slice());
}
// CHECK: FUNC 'test_12' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_f64_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_13"]
unsafe fn test_13() {
    test_args_1(42i8);
}
// CHECK: FUNC 'test_13' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_i8_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_14"]
unsafe fn test_14() {
    test_args_1(42i16);
}
// CHECK: FUNC 'test_14' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_i16_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_15"]
unsafe fn test_15() {
    test_args_1(42i32);
}
// CHECK: FUNC 'test_15' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_i32_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_16"]
unsafe fn test_16() {
    test_args_1(42i64);
}
// CHECK: FUNC 'test_16' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_i64_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_17"]
unsafe fn test_17() {
    test_args_1(42isize);
}
// CHECK: FUNC 'test_17' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_isize_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_18"]
unsafe fn test_18() {
    test_args_1(42u8);
}
// CHECK: FUNC 'test_18' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_u8_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_19"]
unsafe fn test_19() {
    test_args_1(42u16);
}
// CHECK: FUNC 'test_19' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_u16_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_20"]
unsafe fn test_20() {
    test_args_1(42u32);
}
// CHECK: FUNC 'test_20' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_u32_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_21"]
unsafe fn test_21() {
    test_args_1(42u64);
}
// CHECK: FUNC 'test_21' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_u64_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_22"]
unsafe fn test_22() {
    test_args_1(42usize);
}
// CHECK: FUNC 'test_22' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_usize_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_23"]
unsafe fn test_23() {
    test_args_1(42.0f32);
}
// CHECK: FUNC 'test_23' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_f32_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_24"]
unsafe fn test_24() {
    test_args_1(42.0f64);
}
// CHECK: FUNC 'test_24' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_f64_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_25"]
unsafe fn test_25() {
    test_args_1(arg_str!());
}
// CHECK: FUNC 'test_25' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__26_mut_20_str_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_26"]
unsafe fn test_26() {
    test_args_1(ARG_ENUM_BUF_I8);
}
// CHECK: FUNC 'test_26' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_27"]
unsafe fn test_27() {
    test_args_1(ARG_ENUM_BUF_I16);
}
// CHECK: FUNC 'test_27' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_28"]
unsafe fn test_28() {
    test_args_1(ARG_ENUM_BUF_I32);
}
// CHECK: FUNC 'test_28' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_29"]
unsafe fn test_29() {
    test_args_1(ARG_ENUM_BUF_I64);
}
// CHECK: FUNC 'test_29' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_30"]
unsafe fn test_30() {
    test_args_1(ARG_ENUM_BUF_ISIZE);
}
// CHECK: FUNC 'test_30' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_isize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_31"]
unsafe fn test_31() {
    test_args_1(ARG_ENUM_BUF_U8);
}
// CHECK: FUNC 'test_31' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_32"]
unsafe fn test_32() {
    test_args_1(ARG_ENUM_BUF_U16);
}
// CHECK: FUNC 'test_32' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_33"]
unsafe fn test_33() {
    test_args_1(ARG_ENUM_BUF_U32);
}
// CHECK: FUNC 'test_33' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_34"]
unsafe fn test_34() {
    test_args_1(ARG_ENUM_BUF_U64);
}
// CHECK: FUNC 'test_34' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_35"]
unsafe fn test_35() {
    test_args_1(ARG_ENUM_BUF_USIZE);
}
// CHECK: FUNC 'test_35' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_usize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_36"]
unsafe fn test_36() {
    test_args_1(ARG_ENUM_BUF_F32);
}
// CHECK: FUNC 'test_36' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_37"]
unsafe fn test_37() {
    test_args_1(ARG_ENUM_BUF_F64);
}
// CHECK: FUNC 'test_37' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_38"]
unsafe fn test_38() {
    test_args_1(ARG_ENUM_I8);
}
// CHECK: FUNC 'test_38' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_39"]
unsafe fn test_39() {
    test_args_1(ARG_ENUM_I16);
}
// CHECK: FUNC 'test_39' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_40"]
unsafe fn test_40() {
    test_args_1(ARG_ENUM_I32);
}
// CHECK: FUNC 'test_40' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_41"]
unsafe fn test_41() {
    test_args_1(ARG_ENUM_I64);
}
// CHECK: FUNC 'test_41' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_42"]
unsafe fn test_42() {
    test_args_1(ARG_ENUM_ISIZE);
}
// CHECK: FUNC 'test_42' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_isize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_43"]
unsafe fn test_43() {
    test_args_1(ARG_ENUM_U8);
}
// CHECK: FUNC 'test_43' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_44"]
unsafe fn test_44() {
    test_args_1(ARG_ENUM_U16);
}
// CHECK: FUNC 'test_44' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_45"]
unsafe fn test_45() {
    test_args_1(ARG_ENUM_U32);
}
// CHECK: FUNC 'test_45' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_46"]
unsafe fn test_46() {
    test_args_1(ARG_ENUM_U64);
}
// CHECK: FUNC 'test_46' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_47"]
unsafe fn test_47() {
    test_args_1(ARG_ENUM_USIZE);
}
// CHECK: FUNC 'test_47' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_48"]
unsafe fn test_48() {
    test_args_1(ARG_ENUM_F32);
}
// CHECK: FUNC 'test_48' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_49"]
unsafe fn test_49() {
    test_args_1(ARG_ENUM_F64);
}
// CHECK: FUNC 'test_49' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_50"]
unsafe fn test_50() {
    test_args_1(ARG_BUF_I8);
}
// CHECK: FUNC 'test_50' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_51"]
unsafe fn test_51() {
    test_args_1(ARG_BUF_I16);
}
// CHECK: FUNC 'test_51' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_52"]
unsafe fn test_52() {
    test_args_1(ARG_BUF_I32);
}
// CHECK: FUNC 'test_52' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_53"]
unsafe fn test_53() {
    test_args_1(ARG_BUF_I64);
}
// CHECK: FUNC 'test_53' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_i64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_54"]
unsafe fn test_54() {
    test_args_1(ARG_BUF_ISIZE);
}
// CHECK: FUNC 'test_54' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_isize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_55"]
unsafe fn test_55() {
    test_args_1(ARG_BUF_U8);
}
// CHECK: FUNC 'test_55' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_u8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_56"]
unsafe fn test_56() {
    test_args_1(ARG_BUF_U16);
}
// CHECK: FUNC 'test_56' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_u16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_57"]
unsafe fn test_57() {
    test_args_1(ARG_BUF_U32);
}
// CHECK: FUNC 'test_57' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_u32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_58"]
unsafe fn test_58() {
    test_args_1(ARG_BUF_U64);
}
// CHECK: FUNC 'test_58' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_u64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_59"]
unsafe fn test_59() {
    test_args_1(ARG_BUF_USIZE);
}
// CHECK: FUNC 'test_59' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_usize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_60"]
unsafe fn test_60() {
    test_args_1(ARG_BUF_F32);
}
// CHECK: FUNC 'test_60' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_61"]
unsafe fn test_61() {
    test_args_1(ARG_BUF_F64);
}
// CHECK: FUNC 'test_61' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C__5B_f64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_62"]
unsafe fn test_62() {
    test_args_1(ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_62' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_63"]
unsafe fn test_63() {
    test_args_1(ARG_OPT_BUF_I16);
}
// CHECK: FUNC 'test_63' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_64"]
unsafe fn test_64() {
    test_args_1(ARG_OPT_BUF_I32);
}
// CHECK: FUNC 'test_64' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_65"]
unsafe fn test_65() {
    test_args_1(ARG_OPT_BUF_I64);
}
// CHECK: FUNC 'test_65' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_66"]
unsafe fn test_66() {
    test_args_1(ARG_OPT_BUF_ISIZE);
}
// CHECK: FUNC 'test_66' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_67"]
unsafe fn test_67() {
    test_args_1(ARG_OPT_BUF_U8);
}
// CHECK: FUNC 'test_67' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_68"]
unsafe fn test_68() {
    test_args_1(ARG_OPT_BUF_U16);
}
// CHECK: FUNC 'test_68' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_69"]
unsafe fn test_69() {
    test_args_1(ARG_OPT_BUF_U32);
}
// CHECK: FUNC 'test_69' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_70"]
unsafe fn test_70() {
    test_args_1(ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_70' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_71"]
unsafe fn test_71() {
    test_args_1(ARG_OPT_BUF_USIZE);
}
// CHECK: FUNC 'test_71' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_72"]
unsafe fn test_72() {
    test_args_1(ARG_OPT_BUF_F32);
}
// CHECK: FUNC 'test_72' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_73"]
unsafe fn test_73() {
    test_args_1(ARG_OPT_BUF_F64);
}
// CHECK: FUNC 'test_73' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_74"]
unsafe fn test_74() {
    test_args_1(ARG_OPT_I8);
}
// CHECK: FUNC 'test_74' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_75"]
unsafe fn test_75() {
    test_args_1(ARG_OPT_I16);
}
// CHECK: FUNC 'test_75' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_76"]
unsafe fn test_76() {
    test_args_1(ARG_OPT_I32);
}
// CHECK: FUNC 'test_76' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_77"]
unsafe fn test_77() {
    test_args_1(ARG_OPT_I64);
}
// CHECK: FUNC 'test_77' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_78"]
unsafe fn test_78() {
    test_args_1(ARG_OPT_ISIZE);
}
// CHECK: FUNC 'test_78' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_79"]
unsafe fn test_79() {
    test_args_1(ARG_OPT_U8);
}
// CHECK: FUNC 'test_79' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_80"]
unsafe fn test_80() {
    test_args_1(ARG_OPT_U16);
}
// CHECK: FUNC 'test_80' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_81"]
unsafe fn test_81() {
    test_args_1(ARG_OPT_U32);
}
// CHECK: FUNC 'test_81' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_82"]
unsafe fn test_82() {
    test_args_1(ARG_OPT_U64);
}
// CHECK: FUNC 'test_82' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_83"]
unsafe fn test_83() {
    test_args_1(ARG_OPT_USIZE);
}
// CHECK: FUNC 'test_83' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_84"]
unsafe fn test_84() {
    test_args_1(ARG_OPT_F32);
}
// CHECK: FUNC 'test_84' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_85"]
unsafe fn test_85() {
    test_args_1(ARG_OPT_F64);
}
// CHECK: FUNC 'test_85' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_86"]
unsafe fn test_86() {
    test_args_1(ARG_STRUCT_I8);
}
// CHECK: FUNC 'test_86' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_87"]
unsafe fn test_87() {
    test_args_1(ARG_STRUCT_I16);
}
// CHECK: FUNC 'test_87' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_88"]
unsafe fn test_88() {
    test_args_1(ARG_STRUCT_I32);
}
// CHECK: FUNC 'test_88' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_89"]
unsafe fn test_89() {
    test_args_1(ARG_STRUCT_I64);
}
// CHECK: FUNC 'test_89' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_90"]
unsafe fn test_90() {
    test_args_1(ARG_STRUCT_ISIZE);
}
// CHECK: FUNC 'test_90' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_91"]
unsafe fn test_91() {
    test_args_1(ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_91' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_92"]
unsafe fn test_92() {
    test_args_1(ARG_STRUCT_U16);
}
// CHECK: FUNC 'test_92' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_93"]
unsafe fn test_93() {
    test_args_1(ARG_STRUCT_U32);
}
// CHECK: FUNC 'test_93' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_94"]
unsafe fn test_94() {
    test_args_1(ARG_STRUCT_U64);
}
// CHECK: FUNC 'test_94' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_95"]
unsafe fn test_95() {
    test_args_1(ARG_STRUCT_USIZE);
}
// CHECK: FUNC 'test_95' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_96"]
unsafe fn test_96() {
    test_args_1(ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_96' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_97"]
unsafe fn test_97() {
    test_args_1(ARG_STRUCT_F64);
}
// CHECK: FUNC 'test_97' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_98"]
unsafe fn test_98() {
    test_args_1(ARG_STRUCT_BUF_I8);
}
// CHECK: FUNC 'test_98' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_99"]
unsafe fn test_99() {
    test_args_1(ARG_STRUCT_BUF_I16);
}
// CHECK: FUNC 'test_99' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_100"]
unsafe fn test_100() {
    test_args_1(ARG_STRUCT_BUF_I32);
}
// CHECK: FUNC 'test_100' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_101"]
unsafe fn test_101() {
    test_args_1(ARG_STRUCT_BUF_I64);
}
// CHECK: FUNC 'test_101' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_102"]
unsafe fn test_102() {
    test_args_1(ARG_STRUCT_BUF_ISIZE);
}
// CHECK: FUNC 'test_102' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_103"]
unsafe fn test_103() {
    test_args_1(ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_103' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_104"]
unsafe fn test_104() {
    test_args_1(ARG_STRUCT_BUF_U16);
}
// CHECK: FUNC 'test_104' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_105"]
unsafe fn test_105() {
    test_args_1(ARG_STRUCT_BUF_U32);
}
// CHECK: FUNC 'test_105' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_106"]
unsafe fn test_106() {
    test_args_1(ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_106' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_107"]
unsafe fn test_107() {
    test_args_1(ARG_STRUCT_BUF_USIZE);
}
// CHECK: FUNC 'test_107' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_108"]
unsafe fn test_108() {
    test_args_1(ARG_STRUCT_BUF_F32);
}
// CHECK: FUNC 'test_108' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_109"]
unsafe fn test_109() {
    test_args_1(ARG_STRUCT_BUF_F64);
}
// CHECK: FUNC 'test_109' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_110"]
unsafe fn test_110() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_110' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_111"]
unsafe fn test_111() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_ENUM_USIZE);
}
// CHECK: FUNC 'test_111' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_112"]
unsafe fn test_112() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_112' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_113"]
unsafe fn test_113() {
    test_args_2(42.0f32, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_113' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_114"]
unsafe fn test_114() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_BUF_F32);
}
// CHECK: FUNC 'test_114' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_115"]
unsafe fn test_115() {
    test_args_2(ARG_BUF_F32, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_115' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__5B_f32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_116"]
unsafe fn test_116() {
    test_args_2(42.0f32, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_116' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_117"]
unsafe fn test_117() {
    test_args_2(42.0f32, ARG_ENUM_USIZE);
}
// CHECK: FUNC 'test_117' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_118"]
unsafe fn test_118() {
    test_args_2(ARG_BUF_F32, ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_118' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__5B_f32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_119"]
unsafe fn test_119() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_ENUM_BUF_F64);
}
// CHECK: FUNC 'test_119' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_120"]
unsafe fn test_120() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_ENUM_USIZE);
}
// CHECK: FUNC 'test_120' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_121"]
unsafe fn test_121() {
    test_args_2(ARG_ENUM_USIZE, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_121' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_122"]
unsafe fn test_122() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), 42.0f32);
}
// CHECK: FUNC 'test_122' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_f32_3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_123"]
unsafe fn test_123() {
    test_args_2(ARG_BUF_F32, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_123' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__5B_f32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_124"]
unsafe fn test_124() {
    test_args_2(ARG_OPT_BUF_I8, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_124' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_125"]
unsafe fn test_125() {
    test_args_2(ARG_STRUCT_F32, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_125' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_126"]
unsafe fn test_126() {
    test_args_2(ARG_ENUM_USIZE, ARG_BUF_F32);
}
// CHECK: FUNC 'test_126' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_127"]
unsafe fn test_127() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_BUF_F32);
}
// CHECK: FUNC 'test_127' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_128"]
unsafe fn test_128() {
    test_args_2(42.0f32, ARG_BUF_F32);
}
// CHECK: FUNC 'test_128' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_129"]
unsafe fn test_129() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_129' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_130"]
unsafe fn test_130() {
    test_args_2(ARG_OPT_F64, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_130' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_core_3A__3A_option_3A__3A_Option_3C_f64_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_131"]
unsafe fn test_131() {
    test_args_2(ARG_OPT_F64, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_131' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_core_3A__3A_option_3A__3A_Option_3C_f64_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_132"]
unsafe fn test_132() {
    test_args_2(42.0f32, ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_132' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_133"]
unsafe fn test_133() {
    test_args_2(ARG_ENUM_USIZE, ARG_OPT_F64);
}
// CHECK: FUNC 'test_133' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_134"]
unsafe fn test_134() {
    test_args_2(42.0f32, ARG_OPT_F64);
}
// CHECK: FUNC 'test_134' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_135"]
unsafe fn test_135() {
    test_args_2(ARG_ENUM_USIZE, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_135' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_136"]
unsafe fn test_136() {
    test_args_2(ARG_OPT_BUF_I8, ARG_OPT_F64);
}
// CHECK: FUNC 'test_136' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_137"]
unsafe fn test_137() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_OPT_F64);
}
// CHECK: FUNC 'test_137' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_138"]
unsafe fn test_138() {
    test_args_2(ARG_ENUM_USIZE, ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_138' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_139"]
unsafe fn test_139() {
    test_args_2(42.0f32, ARG_ENUM_BUF_F64);
}
// CHECK: FUNC 'test_139' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_f32_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_140"]
unsafe fn test_140() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_STRUCT_F32);
}
// CHECK: FUNC 'test_140' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_141"]
unsafe fn test_141() {
    test_args_2(ARG_BUF_U16.as_mut_slice(), ARG_OPT_BUF_I8);
}
// CHECK: FUNC 'test_141' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_u16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_142"]
unsafe fn test_142() {
    test_args_2(ARG_BUF_F32, ARG_OPT_F64);
}
// CHECK: FUNC 'test_142' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C__5B_f32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_143"]
unsafe fn test_143() {
    test_args_2(ARG_OPT_BUF_I8, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_143' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_144"]
unsafe fn test_144() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_STRUCT_BUF_U64);
}
// CHECK: FUNC 'test_144' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_145"]
unsafe fn test_145() {
    test_args_2(ARG_ENUM_BUF_F64, ARG_OPT_F64);
}
// CHECK: FUNC 'test_145' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_2_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_146"]
unsafe fn test_146() {
    test_args_3(ARG_ENUM_I32, ARG_OPT_BUF_U64, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_146' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_147"]
unsafe fn test_147() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_147' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_148"]
unsafe fn test_148() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_148' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_149"]
unsafe fn test_149() {
    test_args_3(ARG_OPT_I16, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_149' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_150"]
unsafe fn test_150() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_ENUM_BUF_U32);
}
// CHECK: FUNC 'test_150' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_151"]
unsafe fn test_151() {
    test_args_3(
        ARG_BUF_I16.as_mut_slice(),
        ARG_ENUM_BUF_U32,
        ARG_OPT_BUF_U64,
    );
}
// CHECK: FUNC 'test_151' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_152"]
unsafe fn test_152() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_OPT_BUF_U64, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_152' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_153"]
unsafe fn test_153() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_OPT_BUF_U64, ARG_OPT_I16);
}
// CHECK: FUNC 'test_153' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_154"]
unsafe fn test_154() {
    test_args_3(42i8, ARG_ENUM_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_154' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_155"]
unsafe fn test_155() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_BUF_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_155' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_156"]
unsafe fn test_156() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_156' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_157"]
unsafe fn test_157() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_157' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_158"]
unsafe fn test_158() {
    test_args_3(ARG_ENUM_I32, ARG_OPT_BUF_U64, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_158' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_159"]
unsafe fn test_159() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_159' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_160"]
unsafe fn test_160() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_160' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_161"]
unsafe fn test_161() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_ENUM_I32);
}
// CHECK: FUNC 'test_161' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_162"]
unsafe fn test_162() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_162' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_163"]
unsafe fn test_163() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_163' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_164"]
unsafe fn test_164() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_164' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_165"]
unsafe fn test_165() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_BUF_U32, ARG_ENUM_I32);
}
// CHECK: FUNC 'test_165' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_166"]
unsafe fn test_166() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_BUF_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_166' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_167"]
unsafe fn test_167() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_ENUM_I32);
}
// CHECK: FUNC 'test_167' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_168"]
unsafe fn test_168() {
    test_args_3(42i8, ARG_BUF_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_168' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_169"]
unsafe fn test_169() {
    test_args_3(ARG_BUF_I32, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_169' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_170"]
unsafe fn test_170() {
    test_args_3(ARG_BUF_I32, ARG_OPT_BUF_U64, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_170' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_171"]
unsafe fn test_171() {
    test_args_3(ARG_BUF_I32, ARG_OPT_BUF_U64, ARG_OPT_I16);
}
// CHECK: FUNC 'test_171' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_172"]
unsafe fn test_172() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_172' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_173"]
unsafe fn test_173() {
    test_args_3(42i8, ARG_BUF_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_173' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_174"]
unsafe fn test_174() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_ENUM_I32, ARG_BUF_I32);
}
// CHECK: FUNC 'test_174' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_175"]
unsafe fn test_175() {
    test_args_3(ARG_ENUM_I32, ARG_BUF_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_175' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_176"]
unsafe fn test_176() {
    test_args_3(ARG_ENUM_I32, ARG_BUF_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_176' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_177"]
unsafe fn test_177() {
    test_args_3(42i8, ARG_ENUM_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_177' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_178"]
unsafe fn test_178() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_BUF_I32);
}
// CHECK: FUNC 'test_178' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_179"]
unsafe fn test_179() {
    test_args_3(ARG_OPT_BUF_U64, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_179' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_180"]
unsafe fn test_180() {
    test_args_3(42i8, ARG_OPT_BUF_U64, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_180' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_181"]
unsafe fn test_181() {
    test_args_3(ARG_ENUM_I32, ARG_BUF_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_181' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_182"]
unsafe fn test_182() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_182' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_183"]
unsafe fn test_183() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_BUF_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_183' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_184"]
unsafe fn test_184() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_BUF_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_184' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_185"]
unsafe fn test_185() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_BUF_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_185' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_186"]
unsafe fn test_186() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_186' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_187"]
unsafe fn test_187() {
    test_args_3(ARG_ENUM_I32, ARG_BUF_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_187' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_188"]
unsafe fn test_188() {
    test_args_3(42i8, ARG_ENUM_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_188' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_189"]
unsafe fn test_189() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_189' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_190"]
unsafe fn test_190() {
    test_args_3(ARG_OPT_BUF_U64, ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_190' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_191"]
unsafe fn test_191() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_ENUM_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_191' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_192"]
unsafe fn test_192() {
    test_args_3(ARG_ENUM_I32, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_192' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_193"]
unsafe fn test_193() {
    test_args_3(42i8, ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_193' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_194"]
unsafe fn test_194() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_OPT_BUF_U64, ARG_OPT_I16);
}
// CHECK: FUNC 'test_194' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_195"]
unsafe fn test_195() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_BUF_U32, ARG_BUF_I32);
}
// CHECK: FUNC 'test_195' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_196"]
unsafe fn test_196() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_196' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_197"]
unsafe fn test_197() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_ENUM_I32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_197' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_198"]
unsafe fn test_198() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_OPT_BUF_U64, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_198' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_199"]
unsafe fn test_199() {
    test_args_3(42i8, ARG_BUF_I32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_199' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_200"]
unsafe fn test_200() {
    test_args_3(42i8, ARG_OPT_BUF_U64, ARG_OPT_I16);
}
// CHECK: FUNC 'test_200' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_201"]
unsafe fn test_201() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_ENUM_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_201' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_202"]
unsafe fn test_202() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_BUF_U32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_202' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_203"]
unsafe fn test_203() {
    test_args_3(ARG_BUF_I32, ARG_OPT_BUF_U64, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_203' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_204"]
unsafe fn test_204() {
    test_args_3(ARG_BUF_I32, ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_204' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_205"]
unsafe fn test_205() {
    test_args_3(42i8, ARG_BUF_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_205' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_206"]
unsafe fn test_206() {
    test_args_3(
        ARG_BUF_I16.as_mut_slice(),
        ARG_OPT_BUF_U64,
        ARG_STRUCT_BUF_U8,
    );
}
// CHECK: FUNC 'test_206' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_207"]
unsafe fn test_207() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_OPT_BUF_U64, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_207' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_208"]
unsafe fn test_208() {
    test_args_3(ARG_BUF_I32, ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_208' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__5B_i32_3B__20_3_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_209"]
unsafe fn test_209() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_BUF_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_209' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_210"]
unsafe fn test_210() {
    test_args_3(42i8, ARG_ENUM_BUF_U32, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_210' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_211"]
unsafe fn test_211() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_BUF_I32, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_211' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_212"]
unsafe fn test_212() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_BUF_I32);
}
// CHECK: FUNC 'test_212' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_213"]
unsafe fn test_213() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_OPT_I16);
}
// CHECK: FUNC 'test_213' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_214"]
unsafe fn test_214() {
    test_args_3(42i8, ARG_ENUM_I32, ARG_BUF_I32);
}
// CHECK: FUNC 'test_214' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_215"]
unsafe fn test_215() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_I32, ARG_BUF_I32);
}
// CHECK: FUNC 'test_215' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_216"]
unsafe fn test_216() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_216' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_217"]
unsafe fn test_217() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_ENUM_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_217' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_218"]
unsafe fn test_218() {
    test_args_3(ARG_ENUM_I32, ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_218' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_219"]
unsafe fn test_219() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), ARG_ENUM_BUF_U32, ARG_OPT_I16);
}
// CHECK: FUNC 'test_219' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_220"]
unsafe fn test_220() {
    test_args_3(42i8, ARG_STRUCT_U8, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_220' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_221"]
unsafe fn test_221() {
    test_args_3(ARG_ENUM_I32, ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_221' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_222"]
unsafe fn test_222() {
    test_args_3(42i8, ARG_OPT_I16, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_222' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_223"]
unsafe fn test_223() {
    test_args_3(ARG_BUF_I16.as_mut_slice(), 42i8, ARG_OPT_BUF_U64);
}
// CHECK: FUNC 'test_223' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_224"]
unsafe fn test_224() {
    test_args_3(42i8, ARG_OPT_BUF_U64, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_224' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_225"]
unsafe fn test_225() {
    test_args_3(
        ARG_BUF_I16.as_mut_slice(),
        ARG_ENUM_BUF_U32,
        ARG_STRUCT_BUF_U8,
    );
}
// CHECK: FUNC 'test_225' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_i16_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_226"]
unsafe fn test_226() {
    test_args_3(ARG_ENUM_BUF_U32, ARG_BUF_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_226' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__2C__20__5B_i32_3B__20_3_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_227"]
unsafe fn test_227() {
    test_args_3(42i8, ARG_ENUM_I32, ARG_STRUCT_BUF_U8);
}
// CHECK: FUNC 'test_227' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_i8_2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_228"]
unsafe fn test_228() {
    test_args_3(ARG_ENUM_I32, ARG_OPT_BUF_U64, ARG_OPT_I16);
}
// CHECK: FUNC 'test_228' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static

#[no_mangle]
#[link_section = "uprobe/test_229"]
unsafe fn test_229() {
    test_args_3(ARG_OPT_BUF_U64, ARG_OPT_I16, ARG_STRUCT_U8);
}
// CHECK: FUNC 'test_229' type_id={{[0-9]+}} linkage=global
// CHECK: FUNC 'test_args_3_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
