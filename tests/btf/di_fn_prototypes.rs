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

#[no_mangle]
#[link_section = "uprobe/test_2"]
unsafe fn test_2() {
    test_args_1(ARG_BUF_I16.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_3"]
unsafe fn test_3() {
    test_args_1(ARG_BUF_I32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_4"]
unsafe fn test_4() {
    test_args_1(ARG_BUF_I64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_5"]
unsafe fn test_5() {
    test_args_1(ARG_BUF_ISIZE.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_6"]
unsafe fn test_6() {
    test_args_1(ARG_BUF_U8.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_7"]
unsafe fn test_7() {
    test_args_1(ARG_BUF_U16.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_8"]
unsafe fn test_8() {
    test_args_1(ARG_BUF_U32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_9"]
unsafe fn test_9() {
    test_args_1(ARG_BUF_U64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_10"]
unsafe fn test_10() {
    test_args_1(ARG_BUF_USIZE.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_11"]
unsafe fn test_11() {
    test_args_1(ARG_BUF_F32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_12"]
unsafe fn test_12() {
    test_args_1(ARG_BUF_F64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_13"]
unsafe fn test_13() {
    test_args_1(42i8);
}

#[no_mangle]
#[link_section = "uprobe/test_14"]
unsafe fn test_14() {
    test_args_1(42i16);
}

#[no_mangle]
#[link_section = "uprobe/test_15"]
unsafe fn test_15() {
    test_args_1(42i32);
}

#[no_mangle]
#[link_section = "uprobe/test_16"]
unsafe fn test_16() {
    test_args_1(42i64);
}

#[no_mangle]
#[link_section = "uprobe/test_17"]
unsafe fn test_17() {
    test_args_1(42isize);
}

#[no_mangle]
#[link_section = "uprobe/test_18"]
unsafe fn test_18() {
    test_args_1(42u8);
}

#[no_mangle]
#[link_section = "uprobe/test_19"]
unsafe fn test_19() {
    test_args_1(42u16);
}

#[no_mangle]
#[link_section = "uprobe/test_20"]
unsafe fn test_20() {
    test_args_1(42u32);
}

#[no_mangle]
#[link_section = "uprobe/test_21"]
unsafe fn test_21() {
    test_args_1(42u64);
}

#[no_mangle]
#[link_section = "uprobe/test_22"]
unsafe fn test_22() {
    test_args_1(42usize);
}

#[no_mangle]
#[link_section = "uprobe/test_23"]
unsafe fn test_23() {
    test_args_1(42.0f32);
}

#[no_mangle]
#[link_section = "uprobe/test_24"]
unsafe fn test_24() {
    test_args_1(42.1f64);
}

#[no_mangle]
#[link_section = "uprobe/test_25"]
unsafe fn test_25() {
    test_args_1(arg_str!());
}

#[no_mangle]
#[link_section = "uprobe/test_26"]
unsafe fn test_26() {
    test_args_1(ARG_ENUM_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_27"]
unsafe fn test_27() {
    test_args_1(ARG_ENUM_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_28"]
unsafe fn test_28() {
    test_args_1(ARG_ENUM_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_29"]
unsafe fn test_29() {
    test_args_1(ARG_ENUM_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_30"]
unsafe fn test_30() {
    test_args_1(ARG_ENUM_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_31"]
unsafe fn test_31() {
    test_args_1(ARG_ENUM_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_32"]
unsafe fn test_32() {
    test_args_1(ARG_ENUM_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_33"]
unsafe fn test_33() {
    test_args_1(ARG_ENUM_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_34"]
unsafe fn test_34() {
    test_args_1(ARG_ENUM_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_35"]
unsafe fn test_35() {
    test_args_1(ARG_ENUM_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_36"]
unsafe fn test_36() {
    test_args_1(ARG_ENUM_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_37"]
unsafe fn test_37() {
    test_args_1(ARG_ENUM_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_38"]
unsafe fn test_38() {
    test_args_1(ARG_ENUM_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_39"]
unsafe fn test_39() {
    test_args_1(ARG_ENUM_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_40"]
unsafe fn test_40() {
    test_args_1(ARG_ENUM_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_41"]
unsafe fn test_41() {
    test_args_1(ARG_ENUM_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_42"]
unsafe fn test_42() {
    test_args_1(ARG_ENUM_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_43"]
unsafe fn test_43() {
    test_args_1(ARG_ENUM_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_44"]
unsafe fn test_44() {
    test_args_1(ARG_ENUM_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_45"]
unsafe fn test_45() {
    test_args_1(ARG_ENUM_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_46"]
unsafe fn test_46() {
    test_args_1(ARG_ENUM_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_47"]
unsafe fn test_47() {
    test_args_1(ARG_ENUM_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_48"]
unsafe fn test_48() {
    test_args_1(ARG_ENUM_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_49"]
unsafe fn test_49() {
    test_args_1(ARG_ENUM_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_50"]
unsafe fn test_50() {
    test_args_1(ARG_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_51"]
unsafe fn test_51() {
    test_args_1(ARG_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_52"]
unsafe fn test_52() {
    test_args_1(ARG_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_53"]
unsafe fn test_53() {
    test_args_1(ARG_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_54"]
unsafe fn test_54() {
    test_args_1(ARG_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_55"]
unsafe fn test_55() {
    test_args_1(ARG_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_56"]
unsafe fn test_56() {
    test_args_1(ARG_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_57"]
unsafe fn test_57() {
    test_args_1(ARG_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_58"]
unsafe fn test_58() {
    test_args_1(ARG_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_59"]
unsafe fn test_59() {
    test_args_1(ARG_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_60"]
unsafe fn test_60() {
    test_args_1(ARG_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_61"]
unsafe fn test_61() {
    test_args_1(ARG_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_62"]
unsafe fn test_62() {
    test_args_1(ARG_OPT_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_63"]
unsafe fn test_63() {
    test_args_1(ARG_OPT_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_64"]
unsafe fn test_64() {
    test_args_1(ARG_OPT_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_65"]
unsafe fn test_65() {
    test_args_1(ARG_OPT_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_66"]
unsafe fn test_66() {
    test_args_1(ARG_OPT_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_67"]
unsafe fn test_67() {
    test_args_1(ARG_OPT_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_68"]
unsafe fn test_68() {
    test_args_1(ARG_OPT_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_69"]
unsafe fn test_69() {
    test_args_1(ARG_OPT_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_70"]
unsafe fn test_70() {
    test_args_1(ARG_OPT_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_71"]
unsafe fn test_71() {
    test_args_1(ARG_OPT_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_72"]
unsafe fn test_72() {
    test_args_1(ARG_OPT_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_73"]
unsafe fn test_73() {
    test_args_1(ARG_OPT_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_74"]
unsafe fn test_74() {
    test_args_1(ARG_OPT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_75"]
unsafe fn test_75() {
    test_args_1(ARG_OPT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_76"]
unsafe fn test_76() {
    test_args_1(ARG_OPT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_77"]
unsafe fn test_77() {
    test_args_1(ARG_OPT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_78"]
unsafe fn test_78() {
    test_args_1(ARG_OPT_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_79"]
unsafe fn test_79() {
    test_args_1(ARG_OPT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_80"]
unsafe fn test_80() {
    test_args_1(ARG_OPT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_81"]
unsafe fn test_81() {
    test_args_1(ARG_OPT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_82"]
unsafe fn test_82() {
    test_args_1(ARG_OPT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_83"]
unsafe fn test_83() {
    test_args_1(ARG_OPT_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_84"]
unsafe fn test_84() {
    test_args_1(ARG_OPT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_85"]
unsafe fn test_85() {
    test_args_1(ARG_OPT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_86"]
unsafe fn test_86() {
    test_args_1(ARG_STRUCT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_87"]
unsafe fn test_87() {
    test_args_1(ARG_STRUCT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_88"]
unsafe fn test_88() {
    test_args_1(ARG_STRUCT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_89"]
unsafe fn test_89() {
    test_args_1(ARG_STRUCT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_90"]
unsafe fn test_90() {
    test_args_1(ARG_STRUCT_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_91"]
unsafe fn test_91() {
    test_args_1(ARG_STRUCT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_92"]
unsafe fn test_92() {
    test_args_1(ARG_STRUCT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_93"]
unsafe fn test_93() {
    test_args_1(ARG_STRUCT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_94"]
unsafe fn test_94() {
    test_args_1(ARG_STRUCT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_95"]
unsafe fn test_95() {
    test_args_1(ARG_STRUCT_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_96"]
unsafe fn test_96() {
    test_args_1(ARG_STRUCT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_97"]
unsafe fn test_97() {
    test_args_1(ARG_STRUCT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_98"]
unsafe fn test_98() {
    test_args_1(ARG_STRUCT_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_99"]
unsafe fn test_99() {
    test_args_1(ARG_STRUCT_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_100"]
unsafe fn test_100() {
    test_args_1(ARG_STRUCT_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_101"]
unsafe fn test_101() {
    test_args_1(ARG_STRUCT_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_102"]
unsafe fn test_102() {
    test_args_1(ARG_STRUCT_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_103"]
unsafe fn test_103() {
    test_args_1(ARG_STRUCT_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_104"]
unsafe fn test_104() {
    test_args_1(ARG_STRUCT_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_105"]
unsafe fn test_105() {
    test_args_1(ARG_STRUCT_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_106"]
unsafe fn test_106() {
    test_args_1(ARG_STRUCT_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_107"]
unsafe fn test_107() {
    test_args_1(ARG_STRUCT_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_108"]
unsafe fn test_108() {
    test_args_1(ARG_STRUCT_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_109"]
unsafe fn test_109() {
    test_args_1(ARG_STRUCT_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_110"]
unsafe fn test_110() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I16.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_111"]
unsafe fn test_111() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_112"]
unsafe fn test_112() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_113"]
unsafe fn test_113() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_ISIZE.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_114"]
unsafe fn test_114() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U8.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_115"]
unsafe fn test_115() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U16.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_116"]
unsafe fn test_116() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_117"]
unsafe fn test_117() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_118"]
unsafe fn test_118() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_USIZE.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_119"]
unsafe fn test_119() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_F32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_120"]
unsafe fn test_120() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_F64.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_121"]
unsafe fn test_121() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42i8);
}

#[no_mangle]
#[link_section = "uprobe/test_122"]
unsafe fn test_122() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42i16);
}

#[no_mangle]
#[link_section = "uprobe/test_123"]
unsafe fn test_123() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42i32);
}

#[no_mangle]
#[link_section = "uprobe/test_124"]
unsafe fn test_124() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42i64);
}

#[no_mangle]
#[link_section = "uprobe/test_125"]
unsafe fn test_125() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42isize);
}

#[no_mangle]
#[link_section = "uprobe/test_126"]
unsafe fn test_126() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42u8);
}

#[no_mangle]
#[link_section = "uprobe/test_127"]
unsafe fn test_127() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42u16);
}

#[no_mangle]
#[link_section = "uprobe/test_128"]
unsafe fn test_128() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42u32);
}

#[no_mangle]
#[link_section = "uprobe/test_129"]
unsafe fn test_129() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42u64);
}

#[no_mangle]
#[link_section = "uprobe/test_130"]
unsafe fn test_130() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42usize);
}

#[no_mangle]
#[link_section = "uprobe/test_131"]
unsafe fn test_131() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42.0f32);
}

#[no_mangle]
#[link_section = "uprobe/test_132"]
unsafe fn test_132() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), 42.0f64);
}

#[no_mangle]
#[link_section = "uprobe/test_133"]
unsafe fn test_133() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), arg_str!());
}

#[no_mangle]
#[link_section = "uprobe/test_134"]
unsafe fn test_134() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_135"]
unsafe fn test_135() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_136"]
unsafe fn test_136() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_137"]
unsafe fn test_137() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_138"]
unsafe fn test_138() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_139"]
unsafe fn test_139() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_140"]
unsafe fn test_140() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_141"]
unsafe fn test_141() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_142"]
unsafe fn test_142() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_143"]
unsafe fn test_143() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_144"]
unsafe fn test_144() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_145"]
unsafe fn test_145() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_146"]
unsafe fn test_146() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_147"]
unsafe fn test_147() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_148"]
unsafe fn test_148() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_149"]
unsafe fn test_149() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_150"]
unsafe fn test_150() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_151"]
unsafe fn test_151() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_152"]
unsafe fn test_152() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_153"]
unsafe fn test_153() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_154"]
unsafe fn test_154() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_155"]
unsafe fn test_155() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_156"]
unsafe fn test_156() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_157"]
unsafe fn test_157() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_ENUM_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_158"]
unsafe fn test_158() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_159"]
unsafe fn test_159() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_160"]
unsafe fn test_160() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_161"]
unsafe fn test_161() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_162"]
unsafe fn test_162() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_163"]
unsafe fn test_163() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_164"]
unsafe fn test_164() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_165"]
unsafe fn test_165() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_166"]
unsafe fn test_166() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_167"]
unsafe fn test_167() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_168"]
unsafe fn test_168() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_169"]
unsafe fn test_169() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_170"]
unsafe fn test_170() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_171"]
unsafe fn test_171() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_172"]
unsafe fn test_172() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_173"]
unsafe fn test_173() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_174"]
unsafe fn test_174() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_175"]
unsafe fn test_175() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_176"]
unsafe fn test_176() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_177"]
unsafe fn test_177() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_178"]
unsafe fn test_178() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_179"]
unsafe fn test_179() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_180"]
unsafe fn test_180() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_181"]
unsafe fn test_181() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_182"]
unsafe fn test_182() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_183"]
unsafe fn test_183() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_184"]
unsafe fn test_184() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_185"]
unsafe fn test_185() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_186"]
unsafe fn test_186() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_187"]
unsafe fn test_187() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_188"]
unsafe fn test_188() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_189"]
unsafe fn test_189() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_190"]
unsafe fn test_190() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_191"]
unsafe fn test_191() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_192"]
unsafe fn test_192() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_193"]
unsafe fn test_193() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_OPT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_194"]
unsafe fn test_194() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_195"]
unsafe fn test_195() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_196"]
unsafe fn test_196() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_197"]
unsafe fn test_197() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_198"]
unsafe fn test_198() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_199"]
unsafe fn test_199() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_200"]
unsafe fn test_200() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_201"]
unsafe fn test_201() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_202"]
unsafe fn test_202() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_203"]
unsafe fn test_203() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_204"]
unsafe fn test_204() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_205"]
unsafe fn test_205() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_206"]
unsafe fn test_206() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_207"]
unsafe fn test_207() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_208"]
unsafe fn test_208() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_209"]
unsafe fn test_209() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_210"]
unsafe fn test_210() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_211"]
unsafe fn test_211() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_212"]
unsafe fn test_212() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_213"]
unsafe fn test_213() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_214"]
unsafe fn test_214() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_215"]
unsafe fn test_215() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_216"]
unsafe fn test_216() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_217"]
unsafe fn test_217() {
    test_args_2(ARG_BUF_I8.as_mut_slice(), ARG_STRUCT_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_218"]
unsafe fn test_218() {
    test_args_2(ARG_BUF_I16.as_mut_slice(), ARG_BUF_I32.as_mut_slice());
}

#[no_mangle]
#[link_section = "uprobe/test_219"]
unsafe fn test_219() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_220"]
unsafe fn test_220() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_F32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_221"]
unsafe fn test_221() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_222"]
unsafe fn test_222() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_223"]
unsafe fn test_223() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_224"]
unsafe fn test_224() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_225"]
unsafe fn test_225() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_226"]
unsafe fn test_226() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_U8.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_227"]
unsafe fn test_227() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_USIZE.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_228"]
unsafe fn test_228() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_229"]
unsafe fn test_229() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, arg_str!());
}

#[no_mangle]
#[link_section = "uprobe/test_230"]
unsafe fn test_230() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_U16.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_231"]
unsafe fn test_231() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_F64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_232"]
unsafe fn test_232() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_233"]
unsafe fn test_233() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_234"]
unsafe fn test_234() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_U64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_235"]
unsafe fn test_235() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_236"]
unsafe fn test_236() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_237"]
unsafe fn test_237() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_238"]
unsafe fn test_238() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_U8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_239"]
unsafe fn test_239() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_240"]
unsafe fn test_240() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_ISIZE.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_241"]
unsafe fn test_241() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_242"]
unsafe fn test_242() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42i16);
}

#[no_mangle]
#[link_section = "uprobe/test_243"]
unsafe fn test_243() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42u32);
}

#[no_mangle]
#[link_section = "uprobe/test_244"]
unsafe fn test_244() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42.0f64);
}

#[no_mangle]
#[link_section = "uprobe/test_245"]
unsafe fn test_245() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_I8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_246"]
unsafe fn test_246() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_247"]
unsafe fn test_247() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_248"]
unsafe fn test_248() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_I8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_249"]
unsafe fn test_249() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_250"]
unsafe fn test_250() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_I8.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_251"]
unsafe fn test_251() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_252"]
unsafe fn test_252() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_253"]
unsafe fn test_253() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_F64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_254"]
unsafe fn test_254() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_255"]
unsafe fn test_255() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_256"]
unsafe fn test_256() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_257"]
unsafe fn test_257() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_258"]
unsafe fn test_258() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_259"]
unsafe fn test_259() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_260"]
unsafe fn test_260() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_261"]
unsafe fn test_261() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_262"]
unsafe fn test_262() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42i8);
}

#[no_mangle]
#[link_section = "uprobe/test_263"]
unsafe fn test_263() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_264"]
unsafe fn test_264() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_265"]
unsafe fn test_265() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_266"]
unsafe fn test_266() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_F64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_267"]
unsafe fn test_267() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_268"]
unsafe fn test_268() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_269"]
unsafe fn test_269() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_270"]
unsafe fn test_270() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42u16);
}

#[no_mangle]
#[link_section = "uprobe/test_271"]
unsafe fn test_271() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_272"]
unsafe fn test_272() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_273"]
unsafe fn test_273() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42.0f32);
}

#[no_mangle]
#[link_section = "uprobe/test_274"]
unsafe fn test_274() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_I32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_275"]
unsafe fn test_275() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_276"]
unsafe fn test_276() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_277"]
unsafe fn test_277() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_278"]
unsafe fn test_278() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_U8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_279"]
unsafe fn test_279() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_280"]
unsafe fn test_280() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42isize);
}

#[no_mangle]
#[link_section = "uprobe/test_281"]
unsafe fn test_281() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_282"]
unsafe fn test_282() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_283"]
unsafe fn test_283() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_284"]
unsafe fn test_284() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_285"]
unsafe fn test_285() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_286"]
unsafe fn test_286() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_287"]
unsafe fn test_287() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_U8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_288"]
unsafe fn test_288() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_289"]
unsafe fn test_289() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_290"]
unsafe fn test_290() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42u8);
}

#[no_mangle]
#[link_section = "uprobe/test_291"]
unsafe fn test_291() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_292"]
unsafe fn test_292() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_F64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_293"]
unsafe fn test_293() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_I16.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_294"]
unsafe fn test_294() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_I8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_295"]
unsafe fn test_295() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_296"]
unsafe fn test_296() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_297"]
unsafe fn test_297() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_298"]
unsafe fn test_298() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42usize);
}

#[no_mangle]
#[link_section = "uprobe/test_299"]
unsafe fn test_299() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_300"]
unsafe fn test_300() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_I64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_301"]
unsafe fn test_301() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_302"]
unsafe fn test_302() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_303"]
unsafe fn test_303() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_304"]
unsafe fn test_304() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_I8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_305"]
unsafe fn test_305() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_306"]
unsafe fn test_306() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_307"]
unsafe fn test_307() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_308"]
unsafe fn test_308() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_309"]
unsafe fn test_309() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_BUF_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_310"]
unsafe fn test_310() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_311"]
unsafe fn test_311() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_312"]
unsafe fn test_312() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_ENUM_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_313"]
unsafe fn test_313() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42i64);
}

#[no_mangle]
#[link_section = "uprobe/test_314"]
unsafe fn test_314() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_315"]
unsafe fn test_315() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_316"]
unsafe fn test_316() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_OPT_BUF_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_317"]
unsafe fn test_317() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_318"]
unsafe fn test_318() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42u64);
}

#[no_mangle]
#[link_section = "uprobe/test_319"]
unsafe fn test_319() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, ARG_OPT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_320"]
unsafe fn test_320() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_U8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_321"]
unsafe fn test_321() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_BUF_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_322"]
unsafe fn test_322() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_323"]
unsafe fn test_323() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_STRUCT_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_324"]
unsafe fn test_324() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_STRUCT_BUF_F64, 42i32);
}

#[no_mangle]
#[link_section = "uprobe/test_325"]
unsafe fn test_325() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_STRUCT_BUF_F64,
        ARG_ENUM_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_326"]
unsafe fn test_326() {
    test_args_3(
        ARG_BUF_U32.as_mut_slice(),
        ARG_OPT_BUF_U16,
        ARG_BUF_F32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_327"]
unsafe fn test_327() {
    test_args_3(ARG_BUF_U32.as_mut_slice(), ARG_OPT_BUF_U16, ARG_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_328"]
unsafe fn test_328() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_329"]
unsafe fn test_329() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_330"]
unsafe fn test_330() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_331"]
unsafe fn test_331() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_332"]
unsafe fn test_332() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_333"]
unsafe fn test_333() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_334"]
unsafe fn test_334() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_335"]
unsafe fn test_335() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_U64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_336"]
unsafe fn test_336() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_337"]
unsafe fn test_337() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_I64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_338"]
unsafe fn test_338() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_339"]
unsafe fn test_339() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_340"]
unsafe fn test_340() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_341"]
unsafe fn test_341() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_342"]
unsafe fn test_342() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_343"]
unsafe fn test_343() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_344"]
unsafe fn test_344() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_345"]
unsafe fn test_345() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_346"]
unsafe fn test_346() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_347"]
unsafe fn test_347() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42i32);
}

#[no_mangle]
#[link_section = "uprobe/test_348"]
unsafe fn test_348() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_349"]
unsafe fn test_349() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_U8.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_350"]
unsafe fn test_350() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_351"]
unsafe fn test_351() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42u8);
}

#[no_mangle]
#[link_section = "uprobe/test_352"]
unsafe fn test_352() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_353"]
unsafe fn test_353() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_354"]
unsafe fn test_354() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_355"]
unsafe fn test_355() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42i8);
}

#[no_mangle]
#[link_section = "uprobe/test_356"]
unsafe fn test_356() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_357"]
unsafe fn test_357() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42u16);
}

#[no_mangle]
#[link_section = "uprobe/test_358"]
unsafe fn test_358() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_359"]
unsafe fn test_359() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_360"]
unsafe fn test_360() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_361"]
unsafe fn test_361() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_362"]
unsafe fn test_362() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_363"]
unsafe fn test_363() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_364"]
unsafe fn test_364() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42usize);
}

#[no_mangle]
#[link_section = "uprobe/test_365"]
unsafe fn test_365() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_366"]
unsafe fn test_366() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_367"]
unsafe fn test_367() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42u64);
}

#[no_mangle]
#[link_section = "uprobe/test_368"]
unsafe fn test_368() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_OPT_BUF_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_369"]
unsafe fn test_369() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_U32);
}

#[no_mangle]
#[link_section = "uprobe/test_370"]
unsafe fn test_370() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_371"]
unsafe fn test_371() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_F64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_372"]
unsafe fn test_372() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_373"]
unsafe fn test_373() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_374"]
unsafe fn test_374() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_375"]
unsafe fn test_375() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_376"]
unsafe fn test_376() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_377"]
unsafe fn test_377() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_378"]
unsafe fn test_378() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_I32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_379"]
unsafe fn test_379() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42u32);
}

#[no_mangle]
#[link_section = "uprobe/test_380"]
unsafe fn test_380() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42i64);
}

#[no_mangle]
#[link_section = "uprobe/test_381"]
unsafe fn test_381() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_382"]
unsafe fn test_382() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_U8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_383"]
unsafe fn test_383() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_384"]
unsafe fn test_384() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_385"]
unsafe fn test_385() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_OPT_BUF_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_386"]
unsafe fn test_386() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42isize);
}

#[no_mangle]
#[link_section = "uprobe/test_387"]
unsafe fn test_387() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_I8.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_388"]
unsafe fn test_388() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_389"]
unsafe fn test_389() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_390"]
unsafe fn test_390() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_F64);
}

#[no_mangle]
#[link_section = "uprobe/test_391"]
unsafe fn test_391() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_392"]
unsafe fn test_392() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42.0f64);
}

#[no_mangle]
#[link_section = "uprobe/test_393"]
unsafe fn test_393() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_394"]
unsafe fn test_394() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_U32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_395"]
unsafe fn test_395() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_396"]
unsafe fn test_396() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_ISIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_397"]
unsafe fn test_397() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_398"]
unsafe fn test_398() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_ISIZE.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_399"]
unsafe fn test_399() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_I16);
}

#[no_mangle]
#[link_section = "uprobe/test_400"]
unsafe fn test_400() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_F64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_401"]
unsafe fn test_401() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_402"]
unsafe fn test_402() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_U16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_403"]
unsafe fn test_403() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_I64);
}

#[no_mangle]
#[link_section = "uprobe/test_404"]
unsafe fn test_404() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42i16);
}

#[no_mangle]
#[link_section = "uprobe/test_405"]
unsafe fn test_405() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_I32);
}

#[no_mangle]
#[link_section = "uprobe/test_406"]
unsafe fn test_406() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_407"]
unsafe fn test_407() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_ISIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_408"]
unsafe fn test_408() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_BUF_U8);
}

#[no_mangle]
#[link_section = "uprobe/test_409"]
unsafe fn test_409() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_410"]
unsafe fn test_410() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_411"]
unsafe fn test_411() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_USIZE.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_412"]
unsafe fn test_412() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, 42.0f32);
}

#[no_mangle]
#[link_section = "uprobe/test_413"]
unsafe fn test_413() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_I8,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_414"]
unsafe fn test_414() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_415"]
unsafe fn test_415() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_I16.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_416"]
unsafe fn test_416() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_I32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_417"]
unsafe fn test_417() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_ENUM_BUF_F32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_418"]
unsafe fn test_418() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_419"]
unsafe fn test_419() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_F32.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_420"]
unsafe fn test_420() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, arg_str!());
}

#[no_mangle]
#[link_section = "uprobe/test_421"]
unsafe fn test_421() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_F64,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_422"]
unsafe fn test_422() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_U16.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_423"]
unsafe fn test_423() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_424"]
unsafe fn test_424() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_ENUM_USIZE);
}

#[no_mangle]
#[link_section = "uprobe/test_425"]
unsafe fn test_425() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_U64);
}

#[no_mangle]
#[link_section = "uprobe/test_426"]
unsafe fn test_426() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_427"]
unsafe fn test_427() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_U16);
}

#[no_mangle]
#[link_section = "uprobe/test_428"]
unsafe fn test_428() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_I64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_429"]
unsafe fn test_429() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_USIZE,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_430"]
unsafe fn test_430() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_BUF_U64.as_mut_slice(),
    );
}

#[no_mangle]
#[link_section = "uprobe/test_431"]
unsafe fn test_431() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_STRUCT_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_432"]
unsafe fn test_432() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I8,
        ARG_STRUCT_BUF_I16,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_433"]
unsafe fn test_433() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I8, ARG_OPT_BUF_F32);
}

#[no_mangle]
#[link_section = "uprobe/test_434"]
unsafe fn test_434() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I32,
        ARG_STRUCT_BUF_U32,
    );
}

#[no_mangle]
#[link_section = "uprobe/test_435"]
unsafe fn test_435() {
    test_args_4(ARG_STRUCT_I16, ARG_OPT_U32, ARG_OPT_BUF_I32, ARG_BUF_I8);
}

#[no_mangle]
#[link_section = "uprobe/test_436"]
unsafe fn test_436() {
    test_args_4(
        ARG_STRUCT_I16,
        ARG_OPT_U32,
        ARG_OPT_BUF_I32,
        ARG_ENUM_BUF_U64,
    );
}

// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_i64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_isize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_u64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_usize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20__5B_f64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_i8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_i16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_i32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_i64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_isize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_u8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_u16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_u32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_u64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_usize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_f32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_f64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__26_mut_20_str_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_isize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_usize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_isize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_i64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_isize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_u8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_u16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_u32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_u64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_usize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C__5B_f64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_1_3C_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_i16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_i64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_isize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_u8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_u16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_u32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_u64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_usize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20__5B_f64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_i8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_i16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_i32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_i64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_isize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_u8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_u16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_u32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_u64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_usize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_f32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_f64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__26_mut_20_str_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_isize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_usize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_isize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_i64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_isize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_u8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_u16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_u32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_u64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_usize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20__5B_f64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i8_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_2_3C__26_mut_20__5B_i16_5D__2C__20__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_isize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_usize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20_str_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_u32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_isize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_i16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_u32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_f64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_i64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_i8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_f64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_f64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_u16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_u8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_f32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_usize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_isize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_usize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_u8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_usize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_u16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_isize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_i64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20__5B_u64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_u64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_i32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_isize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__2C__20__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_3_3C__26_mut_20__5B_u32_5D__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__2C__20__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_isize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_isize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_usize_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_f64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_i16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_i32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_u8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_u8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_i8_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_u16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u16_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_i32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_usize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_f32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u8_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_u64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_isize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_u32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_usize_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_i64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_u32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_i64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_usize_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_isize_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i8_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_f64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_f64_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_i32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_isize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_i32_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_isize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_f64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u16_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_i16_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_u64_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_isize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u8_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_u64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_usize_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_f32_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i8_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_u64_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_f32_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_i8_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_f32_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20_str_3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_f64_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u16_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_u16_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_usize_3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u64_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__5B_f32_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u16_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_i64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_usize_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20__26_mut_20__5B_u64_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C_f32_3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_i16_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i8_3B__20_3_5D__3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_f32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestStruct_3C__5B_u32_3B__20_3_5D__3E__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__2C__20__5B_i8_3B__20_3_5D__3E_' type_id={{[0-9]+}} linkage=static
// CHECK: FUNC 'test_args_4_3C_di_fn_prototypes_3A__3A_TestStruct_3C_i16_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C_u32_3E__2C__20_core_3A__3A_option_3A__3A_Option_3C__5B_i32_3B__20_3_5D__3E__2C__20_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C_di_fn_prototypes_3A__3A_TestEnum_3C__5B_u64_3B__20_3_5D__3E__3E__3E__3E_' type_id={{[0-9]+}} linkage=static
