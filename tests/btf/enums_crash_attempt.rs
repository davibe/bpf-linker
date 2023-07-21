// attempt at crashing LLVM with enums

#![no_std]
#![no_main]

#[repr(C)]
enum E1<T, U> {
    A,
    B(T, T, U),
    C { x: T, y: U },
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
// CHECK: FUNC 'test_1'

#[no_mangle]
#[link_section = "uprobe/test_2"]
unsafe fn test_2() -> E1<usize, i64> {
    E1::B(42usize, 24, 4242i64)
}
// CHECK: FUNC 'test_2'

#[no_mangle]
#[link_section = "uprobe/test_3"]
unsafe fn test_3() -> E1<i8, i64> {
    E1::C { x: 42, y: 24 }
}
// CHECK: FUNC 'test_3'

#[no_mangle]
#[link_section = "uprobe/test_4"]
unsafe fn test_4() -> Result<E1<i8, i64>, usize> {
    if TEST_SWITCH {
        Ok(E1::B(1, 2, 3))
    } else {
        Err(24)
    }
}
// CHECK: FUNC 'test_4'

#[no_mangle]
#[link_section = "uprobe/test_5"]
unsafe fn test_5() -> Result<E1<i8, i64>, usize> {
    if TEST_SWITCH {
        Ok(E1::C { x: 42, y: 24 })
    } else {
        Err(24)
    }
}
// CHECK: FUNC 'test_5'

#[no_mangle]
#[link_section = "uprobe/test_6"]
unsafe fn test_6() -> Result<E1<*const i8, i64>, usize> {
    if TEST_SWITCH {
        TEST_SWITCH ^= true;
        Ok(E1::C {
            x: core::ptr::null(),
            y: 24,
        })
    } else {
        Err(24)
    }
}
// CHECK: FUNC 'test_6'

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
