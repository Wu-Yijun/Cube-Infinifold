use std::env;


fn main() {
    #[cfg(target_os = "windows")]
    const PATH: &'static str = "PATH";
    #[cfg(target_os = "linux")]
    const PATH: &str = "LD_LIBRARY_PATH";
    #[cfg(target_os = "macos")]
    const PATH: &str = "DYLD_FALLBACK_LIBRARY_PATH";
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    const PATH: &str = "?";

    println!("Hello, world!");
    let path = env::var_os(PATH).unwrap_or_default();
    println!("{PATH}: {:?}", path);

    // add path environment variable
    let new_path = "libs";
    let new_path = format!("{}/{}", env::current_dir().unwrap().display(), new_path);
    println!("Adding path to {PATH}: {new_path}");
    let new_path = std::path::PathBuf::from(new_path);
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    if !paths.contains(&new_path) {
        paths.push(new_path);
    }
    let new_path = env::join_paths(paths).expect("Failed to join paths");
    env::set_var(PATH, new_path);
    println!("{PATH}: {:?}", env::var(PATH).unwrap());
}
