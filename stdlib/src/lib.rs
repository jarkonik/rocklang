#[no_mangle]
pub extern "C" fn printd(x: f32) -> f32 {
    println!("{}", x);
    x
}

#[no_mangle]
pub extern "C" fn vecnew() -> *mut std::ffi::c_void {
    Box::into_raw(Box::new(500.0)) as *mut std::ffi::c_void
}

pub extern "C" fn vecset(vec: *const f64, idx: f64, value: f64) {
    unsafe {
        println!("{:?}", *vec);
    }
}
