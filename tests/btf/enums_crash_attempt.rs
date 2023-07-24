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

static mut TEST_SWITCH: bool = true;

#[no_mangle]
#[link_section = "uprobe/toggle"]
unsafe fn toggle() {
    // not to optimize out if/else
    TEST_SWITCH ^= true
}
// CHECK: FUNC 'toggle'

#[no_mangle]
#[link_section = "uprobe/test_1"]
unsafe fn test_1() -> E1<i8, u8> {
    E1::A
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_1'

#[no_mangle]
#[link_section = "uprobe/test_2"]
unsafe fn test_2() -> E1<usize, i64> {
    E1::B(42usize, 24, 4242i64)
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_2'

#[no_mangle]
#[link_section = "uprobe/test_3"]
unsafe fn test_3() -> E1<i8, i64> {
    E1::C { x: 42, y: 24 }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_3'

#[no_mangle]
#[link_section = "uprobe/test_4"]
unsafe fn test_4() -> Result<E1<i8, i64>, usize> {
    if TEST_SWITCH {
        Ok(E1::B(1, 2, 3))
    } else {
        Err(24)
    }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_4'

#[no_mangle]
#[link_section = "uprobe/test_5"]
unsafe fn test_5() -> Result<E1<i32, i64>, usize> {
    if TEST_SWITCH {
        Ok(E1::C { x: 42, y: 24 })
    } else {
        Err(24)
    }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_5'

#[no_mangle]
#[link_section = "uprobe/test_6"]
unsafe fn test_6() -> Result<E1<*const i8, i64>, usize> {
    if TEST_SWITCH {
        Ok(E1::C {
            x: core::ptr::null(),
            y: 24,
        })
    } else {
        Err(24)
    }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_6'

#[no_mangle]
#[link_section = "uprobe/test_7"]
unsafe fn test_7() -> Result<E2, usize> {
    if TEST_SWITCH {
        Ok(E2::C { x: 42, y: 24 })
    } else {
        Err(24)
    }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_7'

#[no_mangle]
#[link_section = "uprobe/test_8"]
unsafe fn test_8() -> E2 {
    if TEST_SWITCH {
        E2::B(1, 2, 3)
    } else {
        E2::C { x: 42, y: 24 }
    }
}
// CHECK: STRUCT '(anon)'
// CHECK-NEXT: FUNC 'test_8'

/* We now test for NON data carrying enums */

#[no_mangle]
#[link_section = "uprobe/test_9"]
unsafe fn test_9() -> E3 {
    if TEST_SWITCH {
        E3::A
    } else {
        E3::B
    }
}
// CHECK: ENUM 'E3'
// CHECK-NEXT: 'A' val=0
// CHECK-NEXT: 'B' val=1
// CHECK-NEXT: 'C' val=2
// CHECK-NEXT: FUNC 'test_9'

#[repr(u8)]
enum E4 {
    X,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_10"]
unsafe fn test_10() -> E4 {
    if TEST_SWITCH {
        E4::X
    } else {
        E4::Z
    }
}
// CHECK: ENUM 'E4'
// CHECK-NEXT: 'X' val=0
// CHECK-NEXT: 'Y' val=1
// CHECK-NEXT: 'Z' val=2
// CHECK-NEXT: FUNC 'test_10'

#[repr(i16)]
enum E5 {
    X,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_11"]
unsafe fn test_11() -> E5 {
    if TEST_SWITCH {
        E5::X
    } else {
        E5::Z
    }
}
// CHECK: ENUM 'E5'
// CHECK-NEXT: 'X' val=0
// CHECK-NEXT: 'Y' val=1
// CHECK-NEXT: 'Z' val=2
// CHECK-NEXT: FUNC 'test_11'

#[repr(i32)]
enum E6 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_12"]
unsafe fn test_12() -> E6 {
    if TEST_SWITCH {
        E6::X
    } else {
        E6::Z
    }
}
// CHECK: ENUM 'E6'
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_12'

#[repr(i64)]
enum E7 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_13"]
unsafe fn test_13() -> E7 {
    if TEST_SWITCH {
        E7::X
    } else {
        E7::Z
    }
}
// CHECK: ENUM64 'E7'
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_13'

#[repr(isize)]
enum E8 {
    X = -42,
    Y,
    Z,
}

#[no_mangle]
#[link_section = "uprobe/test_14"]
unsafe fn test_14() -> E8 {
    if TEST_SWITCH {
        E8::X
    } else {
        E8::Z
    }
}
// CHECK: ENUM64 'E8'
// CHECK-NEXT: 'X' val=-42
// CHECK-NEXT: 'Y' val=-41
// CHECK-NEXT: 'Z' val=-40
// CHECK-NEXT: FUNC 'test_14'

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
