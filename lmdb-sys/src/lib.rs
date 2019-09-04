#![allow(non_camel_case_types)]
#![deny(warnings)]
#![doc(html_root_url = "https://docs.rs/lmdb-rkv-sys/0.9.0")]

extern crate libc;

#[cfg(unix)]
#[allow(non_camel_case_types)]
pub type mode_t = ::libc::mode_t;
#[cfg(windows)]
#[allow(non_camel_case_types)]
pub type mode_t = ::libc::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
