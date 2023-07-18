// assembly-output: bpf-linker
// compile-flags: --crate-type cdylib -C link-arg=--emit=llvm-ir -C debuginfo=2

// Verify that the linker correctly massages map names.
#![no_std]

// aux-build: loop-panic-handler.rs
extern crate loop_panic_handler;

static mut glob_bool: bool = false;
// CHECK: !DIGlobalVariable(name: "glob_bool"
// CHECK: !DIBasicType(name: "bool"

static mut glob_i8: i8 = 0;
// CHECK: !DIGlobalVariable(name: "glob_i8"
// CHECK: !DIBasicType(name: "i8"

static mut glob_i16: i16 = 0;
// CHECK: !DIGlobalVariable(name: "glob_i16"
// CHECK: !DIBasicType(name: "i16"

static mut glob_i32: i32 = 0;
// CHECK: !DIGlobalVariable(name: "glob_i32"
// CHECK: !DIBasicType(name: "i32"

static mut glob_i64: i64 = 0;
// CHECK: !DIGlobalVariable(name: "glob_i64"
// CHECK: !DIBasicType(name: "i64"

static mut glob_i128: i128 = 0;
// CHECK: !DIGlobalVariable(name: "glob_i128"
// CHECK: !DIBasicType(name: "i128"

static mut glob_isize: isize = 0;
// CHECK: !DIGlobalVariable(name: "glob_isize"
// CHECK: !DIBasicType(name: "isize"

static mut glob_u8: u8 = 0;
// CHECK: !DIGlobalVariable(name: "glob_u8"
// CHECK: !DIBasicType(name: "u8"

static mut glob_u16: u16 = 0;
// CHECK: !DIGlobalVariable(name: "glob_u16"
// CHECK: !DIBasicType(name: "u16"

static mut glob_u32: u32 = 0;
// CHECK: !DIGlobalVariable(name: "glob_u32"
// CHECK: !DIBasicType(name: "u32"

static mut glob_u64: u64 = 0;
// CHECK: !DIGlobalVariable(name: "glob_u64"
// CHECK: !DIBasicType(name: "u64"

static mut glob_u128: u128 = 0;
// CHECK: !DIGlobalVariable(name: "glob_u128"
// CHECK: !DIBasicType(name: "u128"

static mut glob_usize: usize = 0;
// CHECK: !DIGlobalVariable(name: "glob_usize"
// CHECK: !DIBasicType(name: "usize"

static mut glob_f32: f32 = 0.0;
// CHECK: !DIGlobalVariable(name: "glob_f32"
// CHECK: !DIBasicType(name: "f32"

static mut glob_f64: f64 = 0.0;
// CHECK: !DIGlobalVariable(name: "glob_f64"
// CHECK: !DIBasicType(name: "f64"

static mut glob_char: char = 'A';
// CHECK: !DIGlobalVariable(name: "glob_char"
// CHECK: !DIBasicType(name: "char"

static mut glob_str: &'static str = "Aya is awesome";
// CHECK: !DIGlobalVariable(name: "glob_str"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_26_str",

/*
CHECKING OUT TUPLES
*/

static mut glob_tuple_empty: () = ();
// CHECK: !DIGlobalVariable(name: "glob_tuple_empty"
// CHECK: !DIBasicType(name: "()", encoding: DW_ATE_unsigned)

static mut glob_tuple_bool: (bool, bool) = (true, false);
// CHECK: !DIGlobalVariable(name: "glob_tuple_bool"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_bool_2C__20_bool_29_"

static mut glob_tuple_i8: (i8, i8) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_i8"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_i8_2C__20_i8_29_"

static mut glob_tuple_i16: (i16, i16) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_i16"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_i16_2C__20_i16_29_"

static mut glob_tuple_i32: (i32, i32) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_i32"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_i32_2C__20_i32_29_"

static mut glob_tuple_i64: (i64, i64) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_i64"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_i64_2C__20_i64_29_"

static mut glob_tuple_i128: (i128, i128) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_i128"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_i128_2C__20_i128_29_"

static mut glob_tuple_isize: (isize, isize) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_isize"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_isize_2C__20_isize_29_"

static mut glob_tuple_u8: (u8, u8) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_u8"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_u8_2C__20_u8_29_"

static mut glob_tuple_u16: (u16, u16) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_u16"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_u16_2C__20_u16_29_"

static mut glob_tuple_u32: (u32, u32) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_u32"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_u32_2C__20_u32_29_"

static mut glob_tuple_u64: (u64, u64) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_u64"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_u64_2C__20_u64_29_"

static mut glob_tuple_u128: (u128, u128) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_u128"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_u128_2C__20_u128_29_"

static mut glob_tuple_usize: (usize, usize) = (0, 0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_usize"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_usize_2C__20_usize_29_"

static mut glob_tuple_f32: (f32, f32) = (0.0, 0.0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_f32"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_f32_2C__20_f32_29_"

static mut glob_tuple_f64: (f64, f64) = (0.0, 0.0);
// CHECK: !DIGlobalVariable(name: "glob_tuple_f64"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_f64_2C__20_f64_29_"

static mut glob_tuple_char: (char, char) = ('A', 'B');
// CHECK: !DIGlobalVariable(name: "glob_tuple_char"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28_char_2C__20_char_29_"

static mut glob_tuple_str: (&str, &str) = ("Aya", "is awesome");
// CHECK: !DIGlobalVariable(name: "glob_tuple_str"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28__26_str_2C__20__26_str_29_"

/*
CHECKING arrays
 */

static mut glob_array_i8: [i8; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_i8"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_i16: [i16; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_i16"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_i32: [i32; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_i32"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_i64: [i64; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_i64"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_i128: [i128; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_i128"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_isize: [isize; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_isize"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_u8: [u8; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_u8"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_u16: [u16; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_u16"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_u32: [u32; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_u32"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_u64: [u64; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_u64"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_u128: [u128; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_u128"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_usize: [usize; 2] = [0, 0];
// CHECK: !DIGlobalVariable(name: "glob_array_usize"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_f32: [f32; 2] = [0.0, 0.0];
// CHECK: !DIGlobalVariable(name: "glob_array_f32"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_f64: [f64; 2] = [0.0, 0.0];
// CHECK: !DIGlobalVariable(name: "glob_array_f64"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_str: [&'static str; 2] = ["Aya", "is awesome"];
// CHECK: !DIGlobalVariable(name: "glob_array_str"
// CHECK: !DICompositeType(tag: DW_TAG_array_type,

static mut glob_array_au8: [[u8; 3]; 2] = [[0; 3], [0; 3]];
// CHECK: !DIGlobalVariable(name: "glob_array_au8"
// CHECK-NEXT: !DICompositeType(tag: DW_TAG_array_type,
// CHECK-NEXT: !DICompositeType(tag: DW_TAG_array_type,

/*
CHECKING simple functions
 */

static mut glob_fn: fn(usize) -> usize = |x| x;
// CHECK: !DIGlobalVariable(name: "glob_fn"
// CHECK: !DIDerivedType(tag: DW_TAG_pointer_type,
// CHECK: !DISubroutineType(

fn foobar() {}

static mut glob_foobar_fn: fn() = foobar;
// CHECK: !DIGlobalVariable(name: "glob_foobar_fn"
// CHECK: !DIDerivedType(tag: DW_TAG_pointer_type,
// CHECK: !DISubroutineType(

#[no_mangle]
#[link_section = "uprobe/connect"]
pub fn connect() {}
