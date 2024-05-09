use std::{env, process::Command};

#[cfg(target_os = "windows")]
const OS: &str = "windows";
#[cfg(target_os = "linux")]
const OS: &str = "linux";
#[cfg(target_os = "macos")]
const OS: &str = "macos";

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
    // Wait for a while to make sure the cache is updated
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Library cache updated!");
}

pub fn install(){
    let current_path = set_current_dir();
    if OS == "linux" {
        linux_save_custom_search_path(vec![current_path.to_str().unwrap()]);
    }
}