use std::fs::File;
use std::io::Write;
use std::process::Command;

fn write_to_file(output: String) -> Result<(), std::io::Error> {
    let mut file = File::create("output.txt")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn get_birdc_output() -> Result<String, std::io::Error> {
    let output = Command::new("birdc").arg("s").arg("o").arg("s").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    match write_to_file(stdout.clone()) {
        Ok(_) => {
            println!("Command output saved to output.txt");
        }
        Err(e) => {
            println!("Failed to write to file: {}", e);
        }
    }
    Ok(stdout)
}

fn main() {
    match get_birdc_output() {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("Failed to get birdc output: {}", e);
        }
    }
}
