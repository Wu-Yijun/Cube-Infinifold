use std::process::Command;

#[cfg(target_os = "windows")]
const OS: &str = "windows";
#[cfg(target_os = "linux")]
const OS: &str = "linux";
#[cfg(target_os = "macos")]
const OS: &str = "macos";

fn main() {
    if !install() {
        println!("Try to install dependencies again...");
        install();
    }
}

fn install() -> bool {
    // 检查系统中是否已经安装了 ffmpeg
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) if output.status.success() => {
            // 如果已经安装，则打印 "already installed"
            println!("ffmpeg is already installed.");
        }
        _ => {
            println!("Failed to execute ffmpeg command. Seems that ffmpeg is not installed.");
            // 如果没有安装，则安装 ffmpeg
            println!("Installing ffmpeg...");
            // 根据你的系统和包管理器，使用适当的安装命令
            // 这里是以 Debian/Ubuntu 系统为例
            match OS {
                "windows" => {
                    println!("Don't need to install ffmpeg in windows.");
                }
                "linux" => {
                    let install_output = Command::new("sudo")
                        .arg("apt")
                        .arg("install")
                        .arg("ffmpeg")
                        .arg("-y")
                        .spawn()
                        .expect("Failed to execute command")
                        .wait_with_output()
                        .expect("failed to wait on process");

                    if install_output.status.success() {
                        println!("FFmpeg installed successfully.");
                    } else {
                        println!("Failed to install ffmpeg.");
                        println!("Error: {:?}", install_output);
                        return false;
                    }
                }
                "macos" => {
                    Command::new("sudo")
                        .arg("chown")
                        .arg("-R")
                        .arg("$(whoami)")
                        .arg("$(brew --prefix)/*")
                        .spawn()
                        .expect("Failed to chwon")
                        .wait()
                        .expect("Error");
                    let install_output = Command::new("brew")
                        .arg("install")
                        .arg("ffmpeg")
                        .spawn()
                        .expect("Failed to execute command")
                        .wait_with_output()
                        .expect("failed to wait on process");

                    if install_output.status.success() {
                        println!("FFmpeg installed successfully.");
                    } else {
                        println!("Failed to install ffmpeg.");
                        println!("Error: {:?}", install_output);
                        return false;
                    }
                }
                _ => {
                    println!("Unknown system! Cannot install");
                }
            }
        }
    }

    true
}
