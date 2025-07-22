// Minimal WASM test program
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

#[no_mangle]
pub extern "C" fn test_wasm() -> i32 {
    42
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
