// assembly-output: bpf-linker
// compile-flags: --crate-type cdylib -C link-arg=--emit=llvm-ir -C debuginfo=2

// Verify that the linker correctly massages map names.
#![no_std]

// aux-build: loop-panic-handler.rs
extern crate loop_panic_handler;

use core::ops::Add;

trait Dummy {
    fn dummy(&mut self);
}

trait Sum {
    fn sum(&self) -> isize;
}

#[derive(Clone, Copy)]
struct Foo<T> {
    x: T,
}

impl<T> Add for Foo<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
        }
    }
}

#[no_mangle]
#[link_section = "maps"]
static mut FOO: Foo<u32> = Foo { x: 0 };
// CHECK: name: "Foo_3C_u32_3E_"

struct Bar<T> {
    x: T,
}

impl<T> Dummy for Bar<T>
where
    T: Add<Output = T> + Copy,
{
    fn dummy(&mut self) {
        self.x.add(self.x);
    }
}

impl<T> Sum for Bar<T>
where
    T: AsRef<[u8]>,
{
    fn sum(&self) -> isize {
        let mut s = 0;
        self.x.as_ref().iter().for_each(|&b| s += b as isize);
        s
    }
}

enum Blop<X, Y> {
    X(X),
    Y(Y),
}

impl<X, Y> Dummy for Blop<X, Y>
where
    X: Add<Output = X> + Copy,
    Y: Add<Output = Y> + Copy,
{
    fn dummy(&mut self) {
        match *self {
            Self::X(x) => {
                x.add(x);
            }
            Self::Y(y) => {
                y.add(y);
            }
        }
    }
}

#[no_mangle]
#[link_section = "maps"]
static mut BAR: Bar<Foo<u32>> = Bar { x: Foo { x: 0 } };
// CHECK: name: "Bar_3C_di_generics_3A__3A_Foo_3C_u32_3E__3E_"

#[no_mangle]
#[link_section = "uprobe/connect"]
pub fn connect() {
    let _ = const_generic_usize::<42>(1);
    // CHECK: name: "const_generic_usize_3C_42_3E_"
    let _ = const_generic_bool::<true>(42);
    // CHECK: name: "const_generic_bool_3C_true_3E_"
    let _ = const_generic_bool::<false>(42);
    // CHECK: name: "const_generic_bool_3C_false_3E_"
    let _ = const_generic_char::<'A'>(42);
    // name of const generic char does not seem to be predictable but we check it is here
    // CHECK: name: "{{const_generic_char_3C_[[:print:]]+_3E_}}"

    let _ = sum_slice_generic(&[1u8, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_u8_3E_"
    let _ = sum_slice_generic(&[1u16, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_u16_3E_"
    let _ = sum_slice_generic(&[1u32, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_u32_3E_"
    let _ = sum_slice_generic(&[1u64, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_u64_3E_"
    let _ = sum_slice_generic(&[1u128, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_u128_3E_"
    let _ = sum_slice_generic(&[1usize, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_usize_3E_"
    let _ = sum_slice_generic(&[1i8, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_i8_3E_"
    let _ = sum_slice_generic(&[1i16, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_i16_3E_"
    let _ = sum_slice_generic(&[1i32, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_i32_3E_"
    let _ = sum_slice_generic(&[1i64, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_i64_3E_"
    let _ = sum_slice_generic(&[1i128, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_i128_3E_"
    let _ = sum_slice_generic(&[1isize, 2, 3, 4, 5, 6]);
    // CHECK: name: "sum_slice_generic_3C_isize_3E_"
    let _ = sum_slice_generic(&[1f32, 2.0, 3.0, 4.0, 5.0, 6.0]);
    // CHECK: name: "sum_slice_generic_3C_f32_3E_"
    let _ = sum_slice_generic(&[1f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
    // CHECK: name: "sum_slice_generic_3C_f64_3E_"

    // testing for &str
    let _ = sum_bytes("Aya is awesome");
    // using AsRef<str> implies having &str in name
    // CHECK: name: "sum_bytes_3C__26_str_3E_"

    // testing for &[u8]
    let _ = sum_bytes("bpf-linker too !".as_bytes());
    // using AsRef<[u8]> implies having &[u8] in name
    // CHECK: name: "sum_bytes_3C__26__5B_u8_5D__3E_"

    let mut bar = Bar { x: 42usize };
    custom_generic(&mut bar);
    // CHECK: name: "custom_generic_3C_di_generics_3A__3A_Bar_3C_usize_3E__3E_"

    let mut bar = Bar {
        x: Foo { x: 42i32 },
    };
    custom_generic(&mut bar);
    // CHECK: name: "custom_generic_3C_di_generics_3A__3A_Bar_3C_di_generics_3A__3A_Foo_3C_i32_3E__3E__3E_"

    let mut blop: Blop<u8, u8> = Blop::X(42);
    custom_generic(&mut blop);
    // CHECK: name: "custom_generic_3C_di_generics_3A__3A_Blop_3C_u8_2C__20_u8_3E__3E_"

    let mut blop: Blop<u64, isize> = Blop::Y(42);
    custom_generic(&mut blop);
    // CHECK: name: "custom_generic_3C_di_generics_3A__3A_Blop_3C_u64_2C__20_isize_3E__3E_"

    let mut bar = Bar { x: [1u8, 2, 3, 4] };
    let _ = custom_generic_trait(&bar);
    // CHECK: name: "custom_generic_trait_3C_di_generics_3A__3A_Bar_3C__5B_u8_3B__20_4_5D__3E__3E_"
}

#[inline(never)]
fn const_generic_usize<const N: usize>(m: usize) -> usize {
    let mut n = N;
    (0..512).for_each(|_| n += m);
    n
}

#[inline(never)]
fn const_generic_bool<const b: bool>(t: usize) -> usize {
    let mut n = t;
    if b {
        (0..512).for_each(|_| n += 1);
    } else {
        (0..512).for_each(|_| n += 2);
    }
    n
}

#[inline(never)]
fn const_generic_char<const c: char>(b: usize) -> usize {
    let mut n = c as usize;
    (0..512).for_each(|_| n += b);
    n
}

#[inline(never)]
fn sum_slice_generic<T: Add<Output = T> + Copy + Default>(a: &[T]) -> T {
    let mut s: T = T::default();
    a.iter().for_each(|i| {
        s.add(*i);
    });
    s
}

#[inline(never)]
fn sum_bytes<T: AsRef<[u8]>>(a: T) -> usize {
    let mut s = 0;
    a.as_ref().iter().for_each(|&b| s += b as usize);
    s
}

#[inline(never)]
fn custom_generic<T: Dummy>(d: &mut T) {
    d.dummy()
}

#[inline(never)]
fn custom_generic_trait<T: Sum>(s: &T) -> isize {
    s.sum()
}
