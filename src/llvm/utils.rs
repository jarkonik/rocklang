use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

pub fn c_str(mut s: &str) -> Cow<CStr> {
    if s.is_empty() {
        s = "\0";
    }

    if !s.chars().rev().any(|ch| ch == '\0') {
        return Cow::from(CString::new(s).expect("unreachable since null bytes are checked"));
    }

    unsafe { Cow::from(CStr::from_ptr(s.as_ptr() as *const _)) }
}
