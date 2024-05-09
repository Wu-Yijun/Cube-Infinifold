use std::env;

use libloading::{library_filename, Library};

// #[no_mangle]
fn main() {
    println!("\nHello, main fun!");
    print_path();

    load_add();
}

fn print_path() {
    #[cfg(target_os = "windows")]
    const PATH: &str = "PATH";
    #[cfg(target_os = "linux")]
    const PATH: &str = "LD_LIBRARY_PATH";
    #[cfg(target_os = "macos")]
    const PATH: &str = "DYLD_FALLBACK_LIBRARY_PATH";
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    const PATH: &str = "?";

    let path = env::var_os(PATH).unwrap_or_default();
    println!("{PATH}: {:?}", path);
}

fn load_add() {
    let lib = unsafe { Library::new(library_filename("add")).unwrap() };
    let add: libloading::Symbol<fn(i32, i32) -> i32> = unsafe { lib.get(b"add").unwrap() };

    println!("Calling add(1, 2)");
    let result = add(1, 2);
    println!("Result: {}", result);

    lib.close().unwrap();
}
