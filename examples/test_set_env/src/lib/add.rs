#[cfg(target_os = "windows")]
const OS: &str = "windows";
#[cfg(target_os = "linux")]
const OS: &str = "linux";
#[cfg(target_os = "macos")]
const OS: &str = "macos";

/// input: name of the library e.g.
/// output: name of the library with extension based on the OS
/// e.g. "libs/add" -> "libs/add.dll" (windows)
/// e.g. "libs/add" -> "libs/libadd.so" (linux)
/// e.g. "libs/add" -> "libs/libadd.dylib" (macos)
fn get_lib_name(name: &str) -> String {
    let path = name.split("/").collect::<Vec<_>>();
    if path.len() == 1 {
        match OS {
            "windows" => format!("{}.dll", name),
            "linux" => format!("lib{}.so", name),
            "macos" => format!("lib{}.dylib", name),
            _ => {
                println!("Unsupported OS: {:?}", OS);
                name.to_string()
            }
        }
    } else {
        let name = path[path.len() - 1];
        let prefix = path[0..path.len() - 1].join("/");
        match OS {
            "windows" => format!("{}/{}.dll", prefix, name),
            "linux" => format!("{}/lib{}.so", prefix, name),
            "macos" => format!("{}/lib{}.dylib", prefix, name),
            _ => {
                println!("Unsupported OS: {:?}", OS);
                name.to_string()
            }
        }
    }
}

#[no_mangle]
fn add(a: i32, b: i32) -> i32 {
    let lib = unsafe { libloading::Library::new("./libsublib.so").unwrap() };
    let hello: libloading::Symbol<fn() -> i32> = unsafe { lib.get(b"hello").unwrap() };
    println!("Calling hello from sublib.dll");
    hello();
    lib.close().unwrap();

    println!("Adding {} and {}", a, b);
    a + b
}
