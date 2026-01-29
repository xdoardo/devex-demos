//! Some examples of how the attack surface when doing FFI integrations can be reduced with CHERI.

// We can't use `std` on cheriot.
#![cfg_attr(target_family = "cheriot", no_std)]
#![cfg_attr(target_family = "cheriot", feature(core_intrinsics))]

extern crate alloc;
use alloc::string::String;

#[cfg(target_family = "cheriot")]
mod cheriot;

// Example 1: out-of-bounds read
unsafe extern "C" {
    /// There is a bug in this function: it uses strlen to get the length of the payload..
    pub fn json_verify(payload: *const u8) -> bool;
}

fn get_payload() -> String {
    String::from("aaaaaaaa")
}

#[unsafe(no_mangle)]
pub extern "C" fn oob_read() {
    let payload: String = get_payload();

    let mut valid = None;
    cheriot::on_error(
        || {
            valid.replace(unsafe { json_verify(payload.as_ptr()) });
        },
        || {},
    );

    // We are passing `payload.as_ptr()`, but strings are not null-terminated in Rust!
    if valid.is_some_and(|v| v) {
        println!("The payload is valid json");
    } else {
        println!("payload {payload} is not valid json");
    }
}

// Example 2: use after free
unsafe extern "C" {
    /// There is a bug in this function: it uses strlen to get the length of the payload..
    pub fn save(payload: *const u8);
    pub fn load() -> u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn uaf() {
    {
        let data = alloc::vec![42u8];
        unsafe {
            save(data.as_ptr() as *const u8);
        }
    }

    let mut correct = None;
    cheriot::on_error(
        || {
            correct.replace(unsafe { load() });
        },
        || {},
    );

    if correct.is_some_and(|v| v == 42) {
        println!("data correctly saved");
    } else {
        println!("wrong data!");
    }
}
