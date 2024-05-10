use std::process::Command;

fn main() {
    // 检查系统中是否已经安装了 ffmpeg
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) if output.status.success() => {
            // 如果已经安装，则打印 "already installed"
            println!("ffmpeg is already installed.");
        }
        _ => {
            println!("Failed to execute ffmpeg command. Make sure ffmpeg is installed.");
            // 如果没有安装，则安装 ffmpeg
            println!("Installing ffmpeg...");
            // 根据你的系统和包管理器，使用适当的安装命令
            // 这里是以 Debian/Ubuntu 系统为例
            let install_output = Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("ffmpeg")
                .output()
                .expect("Failed to execute command");

            if install_output.status.success() {
                println!("ffmpeg installed successfully.");
            } else {
                println!("Failed to install ffmpeg.");
            }
        }
    }
}
