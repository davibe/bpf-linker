// attempt at crashing LLVM with enums

#![no_std]
#![no_main]

#[repr(C)]
enum E1<T, U> {
    A,
    B(T, T, U),
    C { x: T, y: U },
}

#[repr(C)]
enum E2 {
    A,
    B(i8, u16, usize),
    C { x: isize, y: usize },
}

#[repr(C)]
enum E3 {
    A,
    B,
    C,
}

#[no_mangle]
#[link_section = "uprobe/test_1"]
unsafe fn test_1() -> E1<i8, u8> {
    E1::A
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_1'

#[no_mangle]
#[link_section = "uprobe/test_2"]
unsafe fn test_2() -> E2 {
    E2::B(1, 2, 3)
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_2'

/* We now test for NON data carrying enums */

#[no_mangle]
#[link_section = "uprobe/test_3"]
unsafe fn test_3() -> E3 {
    E3::A
}
// CHECK: ENUM 'E3' encoding=UNSIGNED
// CHECK-NEXT: 'A' val=0
// CHECK-NEXT: 'B' val=1
// CHECK-NEXT: 'C' val=2
// CHECK-NEXT: FUNC 'test_3'

#[repr(u8)]
enum E4 {
    X,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_4"]
unsafe fn test_4() -> E4 {
    E4::X
}
// CHECK: ENUM 'E4' encoding=UNSIGNED
// CHECK-NEXT: 'X' val=0
// CHECK-NEXT: 'Y' val=1
// CHECK-NEXT: 'Z' val=2
// CHECK-NEXT: FUNC 'test_4'

#[repr(i16)]
enum E5 {
    X,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_5"]
unsafe fn test_5() -> E5 {
    E5::Z
}
// CHECK: ENUM 'E5' encoding=SIGNED
// CHECK-NEXT: 'X' val=0
// CHECK-NEXT: 'Y' val=1
// CHECK-NEXT: 'Z' val=2
// CHECK-NEXT: FUNC 'test_5'

#[repr(i32)]
enum E6 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_6"]
unsafe fn test_6() -> E6 {
    E6::X
}
// CHECK: ENUM 'E6' encoding=SIGNED
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_6'

#[repr(i64)]
enum E7 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_7"]
unsafe fn test_7() -> E7 {
    E7::X
}
// CHECK: ENUM64 'E7' encoding=SIGNED
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_7'

#[repr(isize)]
enum E8 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_8"]
unsafe fn test_8() -> E8 {
    E8::X
}
// CHECK: ENUM64 'E8' encoding=SIGNED
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_8'

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
