#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "bindgen"))]
mod ffi;

#[cfg(not(feature = "bindgen"))]
pub use crate::ffi::*;

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn login_auto() {
        unsafe {
            assert_eq!(0, NK_login_auto());
        }
    }

    #[test]
    fn login() {
        let l = CString::new("L").unwrap();
        let p = CString::new("P").unwrap();
        let s = CString::new("S").unwrap();
        let t = CString::new("T").unwrap();
        unsafe {
            // Unconnected
            assert_eq!(0, NK_login(l.as_ptr()));
            assert_eq!(0, NK_login(p.as_ptr()));
            assert_eq!(0, NK_login(s.as_ptr()));
            // Unsupported model
            assert_eq!(0, NK_login(t.as_ptr()));
        }
    }
}
