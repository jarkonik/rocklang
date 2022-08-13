use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    rc::Rc,
};

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn string_from_c_string(ptr: *const i8) -> *const RefCell<String> {
    let c_str = CStr::from_ptr(ptr);
    let rc = Rc::new(RefCell::new(c_str.to_str().unwrap().to_string()));
    Rc::into_raw(rc)
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn c_string_from_string(ptr: *const RefCell<String>) -> *const i8 {
    let rc = Rc::from_raw(ptr);
    let ptr = {
        let string = rc.try_borrow().unwrap();
        let c_string = CString::new(string.to_owned()).unwrap();
        let ptr = c_string.as_ptr();
        std::mem::forget(c_string);
        ptr
    };
    std::mem::forget(rc);
    ptr
}

pub extern "C" fn string(num: f64) -> *const RefCell<String> {
    let rc = Rc::new(RefCell::new(num.to_string()));
    Rc::into_raw(rc)
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn print(ptr: *const RefCell<String>) {
    let rc = Rc::from_raw(ptr);
    {
        let string = rc.try_borrow().unwrap();
        print!("{}", string);
    }
    std::mem::forget(rc);
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn release_string_reference(ptr: *const RefCell<String>) {
    Rc::decrement_strong_count(ptr);
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn inc_string_reference(ptr: *const RefCell<String>) {
    Rc::increment_strong_count(ptr);
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn inc_vec_reference(ptr: *const RefCell<Vec<f64>>) {
    Rc::increment_strong_count(ptr);
}

pub extern "C" fn vec_new() -> *const RefCell<Vec<f64>> {
    let rc = Rc::new(RefCell::new(Vec::new()));
    Rc::into_raw(rc)
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn vec_set(ptr: *const RefCell<Vec<f64>>, idx: f64, val: f64) {
    let rc = Rc::from_raw(ptr);
    {
        let mut vec = rc.try_borrow_mut().unwrap();
        if (idx as usize) >= vec.len() {
            vec.resize((idx as usize) + 1, 0.);
        }
        vec[idx as usize] = val;
    }
    std::mem::forget(rc);
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn vec_get(ptr: *const RefCell<Vec<f64>>, idx: f64) -> f64 {
    let rc = Rc::from_raw(ptr);
    let val = {
        let vec = rc.borrow();
        if (idx as usize) < vec.len() {
            vec[idx as usize]
        } else {
            0.
        }
    };
    std::mem::forget(rc);
    val
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn release_vec_reference(ptr: *const RefCell<Vec<f64>>) {
    Rc::decrement_strong_count(ptr);
}
