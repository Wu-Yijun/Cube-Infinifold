use std::{env, process::Command};

#[cfg(target_os = "windows")]
const OS: &str = "windows";
#[cfg(target_os = "linux")]
const OS: &str = "linux";
#[cfg(target_os = "macos")]
const OS: &str = "macos";

use libloading::{library_filename, Library};

fn set_current_dir() -> std::path::PathBuf {
    let path_exe = env::current_exe().unwrap();
    let path = path_exe.ancestors().nth(1).unwrap();
    let out_path = path.join("libs");
    env::set_current_dir(out_path.clone()).unwrap();
    println!("Output path: {:?}", env::current_dir());
    out_path
}

fn install() {
    let current_path = set_current_dir();
    match OS {
        "windows" => {
            // no need to install ffmpeg

            // const PATH: &str = "PATH";
            // let path = env::var_os(PATH).unwrap_or_default();
            // let mut paths = env::split_paths(&path).collect::<Vec<_>>();
            // if !paths.contains(&current_path) {
            //     paths.push(current_path);
            // }
            // let new_path = env::join_paths(paths).expect("Failed to join paths");
            // env::set_var(PATH, new_path);
            // println!("{PATH}: {:?}", env::var(PATH).unwrap());
        }
        "linux" => {
            // linux
            // install ffmpeg
            let output = Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("ffmpeg")
                .spawn()
                .expect("failed to execute apt-get install ffmpeg");
            println!("FFmpeg installed: {:?}", output);
        }
        "macos" => {
            // macos
            // no need to install ffmpeg
        }
        _ => {
            println!("Unsupported OS: {:?}", OS);
        }
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
    println!("Current Dir {:?}", env::current_dir());
    println!("Library name: {:?}", name);
    let name = env::current_dir().unwrap().join(name);
    println!("Library name: {:?}", name);
    // read number
    // let mut input = String::new();
    // println!("Please input a number:");
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input: i32 = input.trim().parse().unwrap();
    // println!("You input: {}", input);

    let name = get_lib_name("add");

    let lib = unsafe { Library::new(name).unwrap() };
    println!("Library loaded!");
    let add: libloading::Symbol<fn(i32, i32) -> i32> = unsafe { lib.get(b"add").unwrap() };
    let res = add(1, 2);
    println!("Result: {}", res);
    lib.close().unwrap();
}
