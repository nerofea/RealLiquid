// build.rs

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("ffi/ffi_c.cpp")
        .include("include")
        .flag_if_supported("-std=c++17")
        .compile("gdk_ffi")
    // This is important instruction towards Cargo, it is NOT PRINTING TO TERMINAL. 
    println!("cargo:rustc-link-search=native=../path");
    println!("cargo:rustc-link-lib=dylib=gdk_ffi");
}

// Rust FFI Bindings
use std::ffi::{Cstring, CStr};

#[link(name = "gdk_ffi")]
extern "C" {
    fn gdk_init_signer(json: *const libc::c_char);
    fn gdk_get_xpub() -> *const libcc::c_char;
    fn gdk_free_signer();
}

pub fn get_master_xpub(creds_json: &str) -> String {
    let cstr = CString::new(creds_json).unwrap();
    unsafe {
        gdk_init_signer(cstr.as_ptr());
        let ptr = gdk_get_xpub();
        let xpub = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        gdk_free_signer();
        xpub
    }
}

