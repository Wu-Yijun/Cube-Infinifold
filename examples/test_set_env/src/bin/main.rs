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

use std::io::Write;
fn linux_save_custom_search_path(custom_library_path: &str) {
    // 指定要写入的文件路径
    let file_path = "/etc/ld.so.conf.d/my_path.conf";

    // 创建并打开文件
    let mut file = std::fs::File::create(file_path).unwrap();

    // // 读取已有路径
    // let paths = std::fs::read_to_string(file_path).unwrap_or_default();
    // let paths = paths.split('\n').collect::<Vec<_>>();
    // // 如果已有路径中包含自定义路径，则不再写入
    // if paths.contains(&custom_library_path) {
    //     println!("Custom library path already exists in {:?}", file_path);
    //     return;
    // }

    // 写入自定义路径到文件
    writeln!(file, "{}", custom_library_path).unwrap();

    // 如果需要，您可以写入更多路径，例如：
    // writeln!(file, "{}", "/another/custom/library/path")?;

    println!("Custom library path saved to {:?}", file_path);

    let output = Command::new("sudo")
        .arg("ldconfig")
        .spawn()
        .expect("failed to execute ldconfig");

    println!("Library cache updated: {:?}", output);
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
            // set LD_LIBRARY_PATH = current_path
            const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";
            let path = env::var_os(LD_LIBRARY_PATH).unwrap_or_default();
            let mut paths = env::split_paths(&path).collect::<Vec<_>>();
            if !paths.contains(&current_path) {
                paths.push(current_path.clone());
            }
            let new_path = env::join_paths(paths).expect("Failed to join paths");
            env::set_var(LD_LIBRARY_PATH, new_path);
            println!(
                "{LD_LIBRARY_PATH}: {:?}",
                env::var(LD_LIBRARY_PATH).unwrap()
            );

            // update library cache
            linux_save_custom_search_path(current_path.to_str().unwrap());

            // install ffmpeg
            // let output = Command::new("sudo")
            //     .arg("apt-get")
            //     .arg("install")
            //     .arg("ffmpeg")
            //     .spawn()
            //     .expect("failed to execute apt-get install ffmpeg");
            // println!("FFmpeg installed: {:?}", output);
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
    // let name = get_lib_name("add");
    // println!("Current Dir {:?}", env::current_dir());
    // println!("Library name: {:?}", name);
    // let name = env::current_dir().unwrap().join(name);
    // println!("Library name: {:?}", name);
    // read number
    // let mut input = String::new();
    // println!("Please input a number:");
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input: i32 = input.trim().parse().unwrap();
    // println!("You input: {}", input);

    let name = get_lib_name("./add");
    println!("Library name: {:?}", name);

    let lib = unsafe { Library::new(name).unwrap() };
    println!("Library loaded!");
    let add: libloading::Symbol<fn(i32, i32) -> i32> = unsafe { lib.get(b"add").unwrap() };
    let res = add(1, 2);
    println!("Result: {}", res);
    lib.close().unwrap();
}
