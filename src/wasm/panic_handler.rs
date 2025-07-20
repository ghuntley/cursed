//! WASM panic handler
//! 
//! Provides a panic handler for WebAssembly that properly handles panics
//! in the WASM environment by logging to console and aborting gracefully.

use std::panic::PanicInfo;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: &str);
}

#[cfg(target_arch = "wasm32")]
fn console_error(msg: &str) {
    error(msg);
}

#[cfg(not(target_arch = "wasm32"))]
fn console_error(msg: &str) {
    eprintln!("{}", msg);
}

/// WASM-compatible panic handler
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
        format!("CURSED WASM panic: {}", s)
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        format!("CURSED WASM panic: {}", s)
    } else {
        "CURSED WASM panic: unknown error".to_string()
    };

    if let Some(location) = info.location() {
        console_error(&format!(
            "{} at {}:{}:{}",
            msg,
            location.file(),
            location.line(),
            location.column()
        ));
    } else {
        console_error(&msg);
    }

    // In WASM, we need to abort the execution
    wasm_bindgen::throw_str(&msg);
}

/// Initialize WASM panic hook for better error reporting
#[cfg(target_arch = "wasm32")]
pub fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            format!("CURSED WASM panic: {}", s)
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            format!("CURSED WASM panic: {}", s)
        } else {
            "CURSED WASM panic: unknown error".to_string()
        };

        if let Some(location) = info.location() {
            console_error(&format!(
                "{} at {}:{}:{}",
                msg,
                location.file(),
                location.line(),
                location.column()
            ));
        } else {
            console_error(&msg);
        }
    }));
}

/// Initialize WASM panic hook for non-WASM targets (no-op)
#[cfg(not(target_arch = "wasm32"))]
pub fn init_panic_hook() {
    // No-op for non-WASM targets
}
