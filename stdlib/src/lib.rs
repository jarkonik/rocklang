#[no_mangle]
pub extern "C" fn printd(x: f32) -> f32 {
    println!("{}", x);
    x
}

#[no_mangle]
pub extern "C" fn hello() {
    println!("hello");
}
