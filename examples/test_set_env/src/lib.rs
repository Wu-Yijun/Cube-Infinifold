#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    a + b
}
