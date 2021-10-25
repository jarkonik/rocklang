pub extern "C" fn printd(x: f32) -> f32 {
    println!("{}", x);
    x
}

pub extern "C" fn vecnew() -> *const std::ffi::c_void {
    let vec: Vec<f64> = Vec::new();
    Box::into_raw(Box::new(vec)) as *mut std::ffi::c_void
}

pub extern "C" fn vecset(vec: *const Vec<f64>, idx: f64, value: f64) -> *mut std::ffi::c_void {
    let mut v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };

    while v.len() < (idx + 1.0) as usize {
        v.push(0.0);
    }
    v[idx as usize] = value;

    Box::into_raw(v) as *mut std::ffi::c_void
}

pub extern "C" fn vecget(vec: *const Vec<f64>, idx: f64) -> f64 {
    let v = unsafe { Box::from_raw(vec as *mut Vec<f64>) };
    let val = v[idx as usize];
    Box::into_raw(Box::new(v));
    val
}
