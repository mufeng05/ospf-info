use axum::{self, http::StatusCode, routing::get, Router};
use std::io::Write;
use std::net::SocketAddr;
use std::process::Command;
use std::{fs::File, io::Read};

fn write_to_file(output: String) -> Result<(), std::io::Error> {
    let mut file = File::create("./output.txt")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

fn get_birdc_output() -> Result<String, std::io::Error> {
    let output = Command::new("birdc").arg("s").arg("o").arg("s").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Get birdc output successfully");
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

async fn ospf_status() -> (StatusCode, String) {
    match get_birdc_output() {
        Ok(output) => return (StatusCode::OK, output),
        Err(e) => {
            println!("Failed to get birdc output: {}", e);
            let mut file_content = String::new();
            match File::open("output.txt") {
                Ok(mut file) => match file.read_to_string(&mut file_content) {
                    Ok(_) => {
                        return (StatusCode::OK, file_content);
                    }
                    Err(read_err) => {
                        println!("Failed to read from file: {}", read_err);
                    }
                },
                Err(open_err) => {
                    println!("Failed to open file: {}", open_err);
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, String::new());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server on");

    let app = Router::new()
        .route("/get/ospf-info", get(ospf_status))
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:55300").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
