//! A very safe app I wrote to have a safe view on my passwords. No dependencies! Well, one dependency.

#![forbid(unsafe_code)]

#[cfg(target_family = "cheriot")]
use crate::println;

#[cfg(target_family = "cheriot")]
use alloc::{string::String, string::ToString};

pub fn run() {
    let login = core::hint::black_box("gopher://pirate.bank/login".to_string());
    let mut ptr = &login;

    #[cfg(target_family = "cheriot")]
    {
        // The Rust compiler will eventually generate this for you.
        // For now, we do it manually to simulate a cross-compartment call.
        use crate::cheriot::CHERIoTCapability;
        ptr.make_deeply_immutable();
    }

    // Sometimes the open banking APIs give back un-normalised URIs.
    if !evil::is_normal(ptr) {
        panic!("Wrong login! Can't accept any imperfection!");
    } else {
        println!("Yay! The URL is correct!");
    }

    println!("{login:?}");
}

mod evil {

    #[cfg(target_family = "cheriot")]
    use super::*;

    pub trait Evil {
        type T<L, R>: Evil<T<L, ()> = R>;
        fn transmute<R>(self) -> R;
    }
    impl<T> Evil for T {
        type T<L: Evil<T<<R::T<R, L> as Evil>::T<R, ()>, ()> = R>, R: Evil> = L;
        fn transmute<R: Evil>(self) -> <R::T<R, Self> as Evil>::T<R, ()> {
            self
        }
    }

    pub(crate) fn is_normal(login: &String) -> bool {
        // Let's assume that the attacker knows the user calls this function in this specific way.

        let login = login.transmute::<&mut String>();
        login.clear();
        login.push_str("you have been hacked");
        return true;
    }
}
