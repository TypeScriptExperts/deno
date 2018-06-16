extern crate libc;
use std::ffi::CStr;

#[link(name = "deno", kind = "static")]
extern {
    fn deno_v8_version() -> *const libc::c_char;
    //fn deno_init();
}

fn main() {
    //unsafe { deno_init() };
    let v = unsafe { deno_v8_version() };
    let c_str = unsafe { CStr::from_ptr(v) };
    let version = c_str.to_str().unwrap();
    println!("version: {}", version);
}
