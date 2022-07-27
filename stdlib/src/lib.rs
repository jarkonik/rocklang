use std::{cell::RefCell, rc::Rc};

pub extern "C" fn vec_new() -> *const RefCell<Vec<f64>> {
    let vec: Vec<f64> = Vec::new();
    let rc = Rc::new(RefCell::new(vec));
    Rc::into_raw(rc)
}

pub unsafe extern "C" fn vec_reference(vec: *mut RefCell<Vec<f64>>) {
    Rc::increment_strong_count(vec);
}

pub extern "C" fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[allow(clippy::missing_safety_doc)]
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

pub unsafe extern "C" fn vec_get(vec: *mut RefCell<Vec<f64>>, idx: f64) -> f64 {
    let rc = Rc::from_raw(vec);
    let val = {
        let v = rc.borrow();
        v[idx as usize]
    };
    Rc::into_raw(rc);

    val
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn len(vec: *mut RefCell<Vec<f64>>) -> f64 {
    let rc = Rc::from_raw(vec);
    let len = {
        let v = rc.borrow();
        v.len()
    };
    Rc::into_raw(rc);
    len as f64
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn vec_release(vec: *mut RefCell<Vec<f64>>) {
    Rc::decrement_strong_count(vec);
}

// pub extern "C" fn vec_copy(vec: *mut Vec<f64>) -> *mut std::vec::Vec<f64> {
// let v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };
// let new_vec = v.to_vec();
// Box::into_raw(Box::new(v));
// Box::into_raw(Box::new(new_vec))
// }
