// assembly-output: bpf-linker
// compile-flags: --crate-type cdylib -C link-arg=--emit=llvm-ir -C debuginfo=2

// Verify that the linker correctly massages map names.
#![no_std]

// aux-build: loop-panic-handler.rs
extern crate loop_panic_handler;

mod module {
    #[derive(Clone, Copy)]
    pub struct Bar<T> {
        x: T,
        t: (T, T),
        a: [T; 2],
        o: Option<T>,
        s: sub::Sub<T>,
    }

    impl<T> Bar<T>
    where
        T: Copy,
    {
        pub const fn new(v: T) -> Self {
            Self {
                x: v,
                t: (v, v),
                a: [v, v],
                o: Some(v),
                s: sub::Sub(v),
            }
        }
    }

    mod sub {
        #[derive(Clone, Copy)]
        pub struct Sub<T>(pub T);
    }
}

static mut glob_bar: module::Bar<u8> = module::Bar::new(0);
// CHECK: !DIGlobalVariable(name: "glob_bar"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "Bar_3C_u8_3E_"
// CHECK: !DINamespace(name: "module"
// CHECK: !DIDerivedType(tag: DW_TAG_member, name: "o"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type,
// CHECK: !DINamespace(name: "option", scope: !14)
// CHECK: !DINamespace(name: "core", scope: null)

static mut glob_foo: module::Bar<[u8; 64]> = module::Bar::new([0; 64]);
// CHECK: !DIGlobalVariable(name: "glob_foo"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "Bar_3C__5B_u8_3B__20_64_5D__3E_"
// CHECK: !DIDerivedType(tag: DW_TAG_member, name: "o"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type,

static mut glob_array: [module::Bar<([u8; 2], (u8, usize))>; 12] =
    [module::Bar::new(([0, 0], (0, 0))); 12];
// CHECK: !DIGlobalVariable(name: "glob_array"
// CHECK: !DICompositeType(tag: DW_TAG_array_type
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "Bar_3C__28__5B_u8_3B__20_2_5D__2C__20__28_u8_2C__20_usize_29__29__3E_"
// CHECK: !DIDerivedType(tag: DW_TAG_member, name: "x"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28__5B_u8_3B__20_2_5D__2C__20__28_u8_2C__20_usize_29__29_"
// CHECK: !DIDerivedType(tag: DW_TAG_member, name: "t"
// CHECK: !DICompositeType(tag: DW_TAG_structure_type, name: "_28__28__5B_u8_3B__20_2_5D__2C__20__28_u8_2C__20_usize_29__29__2C__20__28__5B_u8_3B__20_2_5D__2C__20__28_u8_2C__20_usize_29__29__29_"

#[no_mangle]
#[link_section = "uprobe/connect"]
pub fn connect() {}
