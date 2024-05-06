mod ffmpeg;
mod font;

/// this application is used to check the cube-infinifold executable
/// and its output in github actions
fn main() {
    println!("Hello, world!");

    font::main();

    ffmpeg::main();

    println!("All checks passed!");

    std::process::exit(0);
}
