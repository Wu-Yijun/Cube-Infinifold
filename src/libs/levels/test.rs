
#[no_mangle]
pub static mut vvv: [bool; 4] = [true, false, true, true];
#[no_mangle]
pub static mut v: bool = false;

#[no_mangle]
pub fn get_id3() -> Vec<i32> {
    vec![1, 2, 3, 4]
}

#[no_mangle]
pub fn new() {
    unsafe { vvv = [true, false, true, true] };
    unsafe { v = true };
}
