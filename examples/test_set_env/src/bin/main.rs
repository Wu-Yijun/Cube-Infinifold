use std::{env, process::Command};

#[cfg(target_os = "windows")]
const OS: &str = "windows";
#[cfg(target_os = "linux")]
const OS: &str = "linux";
#[cfg(target_os = "macos")]
const OS: &str = "macos";

use libloading::Library;

fn set_current_dir() -> std::path::PathBuf {
    let path_exe = env::current_exe().unwrap();
    let path = path_exe.ancestors().nth(1).unwrap();
    let out_path = path.join("libs");
    env::set_current_dir(out_path.clone()).unwrap();
    println!("Output path: {:?}", env::current_dir());
    out_path
}

fn linux_save_custom_search_path(mut custom_library_path: Vec<&str>) {
    const FILE_PATH: &str = "/etc/ld.so.conf.d/cube_infinifold_libs.conf";
    // sort and remove duplicates
    custom_library_path.sort();
    custom_library_path.dedup();
    let custom_library_path = custom_library_path.join("\n");

    let file_content = std::fs::read_to_string(FILE_PATH).unwrap_or_default();
    if file_content == custom_library_path {
        return;
    }

    println!("Installing custom search path...");
    std::fs::write(FILE_PATH, custom_library_path)
        .expect("Unable to write file, try running with sudo!");
    Command::new("sudo")
        .arg("ldconfig")
        .spawn()
        .expect("failed to execute ldconfig");
    // 等待一段时间以确保缓存已更新
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Library cache updated!");
}

fn install() {
    let current_path = set_current_dir();
    if OS == "linux" {
        linux_save_custom_search_path(vec![current_path.to_str().unwrap()]);
    }
}

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

fn main() {
    install();

    let name = get_lib_name("add");
    println!("Library name: {:?}", name);

    let lib = unsafe { Library::new(name).unwrap() };
    println!("Library loaded!");
    let add: libloading::Symbol<fn(i32, i32) -> i32> = unsafe { lib.get(b"add").unwrap() };
    let res = add(1, 2);
    println!("Result: {}", res);
    lib.close().unwrap();
}
