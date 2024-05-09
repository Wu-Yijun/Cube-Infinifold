use std::env;

use libloading::{library_filename, Library};

fn set_path(always_path: bool) {
    #[cfg(target_os = "windows")]
    const PATH: &str = "PATH";
    #[cfg(target_os = "linux")]
    const PATH: &str = "LD_LIBRARY_PATH";
    #[cfg(target_os = "macos")]
    const PATH: &str = "DYLD_FALLBACK_LIBRARY_PATH";
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    const PATH: &str = "?";

    let PP: &str = if always_path { "PATH" } else { PATH };

    println!("Hello, world!");
    let path = env::var_os(if always_path { "PATH" } else { PP }).unwrap_or_default();
    println!("{PP}: {:?}", path);

    // add path environment variable
    let new_path = "libs/";
    let new_path = format!("{}/{}", env::current_dir().unwrap().display(), new_path);
    println!("Adding path to {PP}: {new_path}");
    let new_path = std::path::PathBuf::from(new_path);
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    if !paths.contains(&new_path) {
        paths.push(new_path);
    }
    let new_path = env::join_paths(paths).expect("Failed to join paths");
    env::set_var(PP, new_path);
    println!("{PP}: {:?}", env::var(PP).unwrap());
    
}

unsafe fn load_lib() {
    let lib = Library::new(library_filename("add")).unwrap();
    let add: libloading::Symbol<fn(i32, i32) -> i32> = lib.get(b"add").unwrap();

    println!("Calling add(1, 2)");
    let result = add(1, 2);
    println!("Result: {}", result);

    lib.close().unwrap();
}

fn main() {
    let output = std::process::Command::new("export LD_LIBRARY_PATH=:/home/runner/work/Cube-Infinifold/Cube-Infinifold/target/debug/libs")
        .output()
        .expect("Failed to execute command");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    // set_path(false);
    // set_path(true);
    unsafe { load_lib() };
}
