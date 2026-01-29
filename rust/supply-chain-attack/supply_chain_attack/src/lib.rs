//! An example of how soundness holes in Rust can be leveraged in malicious third-party code to
//! perform supply-chain attacks, even if no unsafe is used.

// We can't use `std` on cheriot.
#![cfg_attr(target_family = "cheriot", no_std)]
#![cfg_attr(target_family = "cheriot", allow(internal_features))]
#![cfg_attr(target_family = "cheriot", feature(core_intrinsics))]

extern crate alloc;

#[cfg(target_family = "cheriot")]
mod cheriot;

// All the code for this example lives in this module, so that we can forbid unsafe code (instead
// of 'deny'-ing it). We still have to use unsafe bits to interact with CHERIoT and expose
// entrypoints to the example itself.
pub mod safe;

#[unsafe(no_mangle)]
pub extern "C" fn do_it() {
    safe::run();
}
