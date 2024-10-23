use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // 执行系统命令
    let output = Command::new("birdc")
        .arg("s")
        .arg("o")
        .arg("s")
        .output()
        .expect("Failed to execute command");

    // 将输出转换为字符串
    let stdout = String::from_utf8_lossy(&output.stdout);

    // 打开文件并写入输出
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(stdout.as_bytes())
        .expect("Failed to write to file");

    println!("{}", stdout);
    println!("Command output saved to output.txt");
}
