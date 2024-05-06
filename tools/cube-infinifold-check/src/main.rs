mod ffmpeg;
mod font;

/// this application is used to check the cube-infinifold executable
/// and its output in github actions
fn main() {
    println!("Hello, world!");

    join_path("libs");

    font::main();

    ffmpeg::main();

    println!("All checks passed!");

    std::process::exit(0);
}

use std::{env, path::PathBuf};
fn join_path(relative_path: &str) {
    // 获取当前的PATH环境变量
    let mut paths = match env::var_os("PATH") {
        Some(val) => env::split_paths(&val).collect::<Vec<_>>(),
        None => Vec::new(),
    };

    let path_exe = std::env::current_exe().expect("Failed to get current exe");
    let path = path_exe.ancestors().nth(1).unwrap();
    // 将新路径添加到PATH环境变量中
    let new_path_buf = PathBuf::from(format!("{}/{}", path.display(), relative_path));
    if !paths.contains(&new_path_buf) {
        paths.push(new_path_buf);
    }

    // 生成新的PATH环境变量字符串
    let new_path_str = env::join_paths(paths).expect("Failed to join paths");

    // 设置新的PATH环境变量
    env::set_var("PATH", new_path_str);

    // 打印新的PATH环境变量
    println!("PATH: {:?}", env::var("PATH").unwrap());
}
