//! See memory_safety/example.cc at the top-level of this repository for more details.
//! This crate is a naive reproduction of the same issues, mainly using unsafe Rust.

// Of course, we can't use `std`.
#![no_std]

use core::hint::black_box;

extern crate alloc;

#[cfg(not(target_family = "cheriot"))]
compile_error!("We only want to produce code for CHERIoT now!");

mod cheriot;

#[unsafe(no_mangle)]
pub extern "C" fn spatial_safety_error_stack() -> u8 {
    let vec: [u8; 3] = [1, 2, 3];
    unsafe {
        // Use black_box to stop rust to optimize it to a constant, since `len` is known at
        // compile-time.
        let len = black_box(vec.len());

        // Use get_unchecked to get a 'raw' pointer that points to the (oob) slot in the vector.
        // We use `get_unchecked` instead of `get` checks the bounds and returns an `Option`.
        //
        // In debug mode this would cause a panic as well, iff. debug assertions are enabled.
        let ptr = vec.get_unchecked(len) as *const u8;

        // We don't use vec[3] directly because Rust automatically generates bounds checks and
        // would normally panic at runtime; this, instead, is proper undefined behaviour in
        // release mode.
        *ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn spatial_safety_error_global() -> u8 {
    static GLOBAL_VEC: [u8; 3] = [1, 2, 3];
    unsafe {
        let len = black_box(GLOBAL_VEC.len());

        let ptr = GLOBAL_VEC.get_unchecked(len);

        *ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn spatial_safety_error_heap() -> u8 {
    let vec = alloc::vec![0, 1, 2, 3, 4, 5, 6, 7];
    unsafe {
        let len = black_box(vec.len());
        let ptr = vec.get_unchecked(len);
        *ptr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn use_after_free() -> u8 {
    let vec: alloc::vec::Vec<u8> = alloc::vec![1, 2, 3];
    unsafe {
        let item = {
            let len = black_box(vec.len());
            vec.get_unchecked(len) as *const u8
        };

        drop(vec);
        *item
    }
}
