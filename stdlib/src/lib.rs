use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    rc::Rc,
};

pub extern "C" fn vec_new() -> *const RefCell<Vec<f64>> {
    let vec: Vec<f64> = Vec::new();
    let rc = Rc::new(RefCell::new(vec));
    Rc::into_raw(rc)
}

/// # Safety
/// Accesses raw pointer
pub unsafe extern "C" fn vec_reference(vec: *mut RefCell<Vec<f64>>) {
    Rc::increment_strong_count(vec);
}

pub extern "C" fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

/// # Safety
/// Accesses raw pointer
pub unsafe extern "C" fn vec_mut(vec: *const RefCell<Vec<f64>>, idx: f64, value: f64) {
    let rc = Rc::from_raw(vec);

    {
        let mut v = rc.borrow_mut();
        while v.len() <= idx as usize {
            v.push(0.0);
        }
        v[idx as usize] = value;
    }
    Rc::into_raw(rc);
}

/// # Safety
/// Accesses raw pointer
pub unsafe extern "C" fn vec_get(vec: *mut RefCell<Vec<f64>>, idx: f64) -> f64 {
    let rc = Rc::from_raw(vec);
    let val = {
        let v = rc.borrow();
        v[idx as usize]
    };
    Rc::into_raw(rc);

    val
}

/// # Safety
/// Accesses raw pointer
pub unsafe extern "C" fn len(vec: *mut RefCell<Vec<f64>>) -> f64 {
    let rc = Rc::from_raw(vec);
    let len = {
        let v = rc.borrow();
        v.len()
    };
    Rc::into_raw(rc);
    len as f64
}

/// # Safety
/// Accesses raw pointer
pub unsafe extern "C" fn vec_release(vec: *mut RefCell<Vec<f64>>) {
    Rc::decrement_strong_count(vec);
}

#[no_mangle]
pub extern "C" fn string(num: f64) -> *const i8 {
    CString::into_raw(CString::new(num.to_string()).unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn print(ptr: *mut i8) {
    let c_str = CStr::from_ptr(ptr);
    let str = c_str.to_str().unwrap();
    print!("{}", str);
}
