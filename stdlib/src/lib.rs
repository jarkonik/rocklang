pub extern "C" fn vecnew() -> *mut std::vec::Vec<f64> {
    let vec: Vec<f64> = Vec::new();
    Box::into_raw(Box::new(vec))
}

pub extern "C" fn vecset(vec: *mut Vec<f64>, idx: f64, value: f64) -> *mut std::vec::Vec<f64> {
    let mut v = unsafe { Box::from_raw(vec) };

    while v.len() <= idx as usize {
        v.push(0.0);
    }
    v[idx as usize] = value;

    Box::into_raw(v)
}

pub extern "C" fn vecget(vec: *mut Vec<f64>, idx: f64) -> f64 {
    let v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };
    let val = v[idx as usize];
    Box::into_raw(Box::new(v));
    val
}

pub extern "C" fn vecfree(vec: *mut Vec<f64>) {
    unsafe {
        Box::from_raw(vec);
    }
}
