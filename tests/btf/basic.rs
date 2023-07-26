// basic btf test

#![no_std]
#![no_main]

#[no_mangle]
#[link_section = "uprobe/connect"]
pub fn connect() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

// We check the BTF dump out of bpftool
// CHECK: FUNC 'connect' type_id=1 linkage=global
