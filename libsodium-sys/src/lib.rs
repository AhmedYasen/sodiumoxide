#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)] // we can't control bindgen output to make clippy happy

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
extern crate libc;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod libc {
    pub type c_void = u8;
    pub type c_int = i32;
    pub type c_ulonglong = u64;
    pub type c_char = i8;
    pub type size_t = usize;
    pub type uint64_t = u64;
}

use libc::{c_void, c_int, c_ulonglong, c_char, size_t, uint64_t};

mod sodium_bindings;
pub use sodium_bindings::*;
