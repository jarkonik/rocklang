pub extern "C" fn vecnew() -> *mut std::vec::Vec<f64> {
    let vec: Vec<f64> = Vec::new();
    Box::into_raw(Box::new(vec))
}

pub extern "C" fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn vecset(
    vec: *mut Vec<f64>,
    idx: f64,
    value: f64,
) -> *mut std::vec::Vec<f64> {
    let v = Box::from_raw(vec);
    let mut new_vec = v.to_vec();
    Box::into_raw(v);

    while new_vec.len() <= idx as usize {
        new_vec.push(0.0);
    }
    new_vec[idx as usize] = value;

    Box::into_raw(Box::new(new_vec))
}

pub extern "C" fn vecget(vec: *mut Vec<f64>, idx: f64) -> f64 {
    let v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };
    let val = v[idx as usize];
    Box::into_raw(Box::new(v));
    val
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn len(vec: *mut Vec<f64>) -> f64 {
    let v = Box::from_raw(vec as *mut Vec<f64>);
    let length = v.len() as f64;
    Box::into_raw(Box::new(v));
    length
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn vecfree(vec: *mut Vec<f64>) {
    Box::from_raw(vec);
}

pub extern "C" fn veccopy(vec: *mut Vec<f64>) -> *mut std::vec::Vec<f64> {
    let v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };
    let new_vec = v.to_vec();
    Box::into_raw(Box::new(v));
    Box::into_raw(Box::new(new_vec))
}
