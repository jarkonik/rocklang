use std::{cell::RefCell, ffi::CStr, rc::Rc};

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn string_from_c_string(ptr: *const i8) -> *const RefCell<String> {
    let c_str = CStr::from_ptr(ptr);
    let rc = Rc::new(RefCell::new(c_str.to_str().unwrap().to_string()));
    Rc::into_raw(rc)
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
        while vec.len() <= idx as usize {
            vec.push(0.);
        }
        vec[idx as usize] = val;
    }
    std::mem::forget(rc);
}

/// # Safety
///
/// loads raw ptr
pub unsafe extern "C" fn release_vec_reference(ptr: *const RefCell<Vec<f64>>) {
    Rc::decrement_strong_count(ptr);
}
