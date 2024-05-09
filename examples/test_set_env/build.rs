fn main() {
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        ":/home/runner/work/Cube-Infinifold/Cube-Infinifold/target/debug/libs"
    );
}
