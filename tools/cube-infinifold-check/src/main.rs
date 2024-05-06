mod ffmpeg;

/// this application is used to check the cube-infinifold executable
/// and its output in github actions
fn main() {
    println!("Hello, world!");
    ffmpeg::main();
    std::process::exit(0);
}
