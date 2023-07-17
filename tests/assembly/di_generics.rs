// assembly-output: bpf-linker
// compile-flags: --crate-type cdylib -C link-arg=--emit=llvm-ir -C debuginfo=2

// Verify that the linker correctly massages map names.
#![no_std]

// aux-build: loop-panic-handler.rs
extern crate loop_panic_handler;

struct Foo<T> {
    x: T,
}

#[no_mangle]
#[link_section = "maps"]
static mut FOO: Foo<u32> = Foo { x: 0 };

struct Bar<T> {
    x: T,
}

#[no_mangle]
#[link_section = "maps"]
static mut BAR: Bar<Foo<u32>> = Bar { x: Foo { x: 0 } };

// NOTE(vadorovsky): I couldn't come up with any simpler example of function
// with generic which wouldn't get inlined.

#[no_mangle]
#[link_section = "uprobe/connect"]
pub fn connect() {
    let _ = my_function(1, 2);
    let _ = my_function(3, 4);
    let _ = my_function(5, 6);
    let _ = my_function(7, 8);

    let _ = const_generic_usize::<42>(1);
    let _ = const_generic_bool::<true>(42);
    let _ = const_generic_bool::<false>(42);
    let _ = const_generic_char::<'A'>(42);
}

pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for u32 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self + other
    }
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
pub fn my_function<T: Add<Output = T> + Copy>(x: T, y: T) -> T {
    x.add(y)
        .add(x)
        .add(y)
        .add(x)
        .add(y)
        .add(x)
        .add(y)
        .add(x)
        .add(y)
        .add(x)
        .add(y)
        .add(x)
        .add(y)
}

// CHECK: name: "Foo_3C_u32_3E_"
// CHECK: name: "Bar_3C_di_generics_3A__3A_Foo_3C_u32_3E__3E_"
// CHECK: name: "my_function_3C_u32_3E_"
// CHECK: name: "const_generic_usize_3C_42_3E_"
// CHECK: name: "const_generic_bool_3C_true_3E_"
// CHECK: name: "const_generic_bool_3C_false_3E_"
// name of const generic char does not seem to be predictable but we check it is here
// CHECK: name: "{{const_generic_char_3C_[[:print:]]+_3E_}}"
