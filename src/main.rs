use axum::{self, http::StatusCode, routing::get, Router};
use std::io::Write;
use std::process::Command;
use std::{fs::File, io::Read};
use flexi_logger::Logger;
use log::{info, warn, error};

async fn write_to_file(output: String) -> Result<(), std::io::Error> {
    let mut file = File::create("ospf-info.txt")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

async fn get_birdc_output() -> Result<String, std::io::Error> {
    let output = Command::new("birdc").arg("s").arg("o").arg("s").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    info!("Get birdc output successfully");
    match write_to_file(stdout.clone()).await {
        Ok(_) => {
            info!("Command output saved to ospf-info.txt");
        }
        Err(e) => {
            warn!("Failed to write to file: {}", e);
        }
    }
    Ok(stdout)
}

async fn ospf_status() -> (StatusCode, String) {
    match get_birdc_output().await {
        Ok(output) => return (StatusCode::OK, output),
        Err(e) => {
            warn!("Failed to get birdc output: {}", e);
            let mut file_content = String::new();
            match File::open("ospf-info.txt") {
                Ok(mut file) => match file.read_to_string(&mut file_content) {
                    Ok(_) => {
                        info!("Get output from file successfully");
                        return (StatusCode::OK, file_content);
                    }
                    Err(read_err) => {
                        error!("Failed to read from file: {}", read_err);
                    }
                },
                Err(open_err) => {
                    error!("Failed to open file: {}", open_err);
                }
            }
            return (StatusCode::INTERNAL_SERVER_ERROR, String::new());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_str("info")?.start()?;
    info!("Starting server on");
    let app = Router::new()
        .route("/get/ospf-info", get(ospf_status))
        .into_make_service();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:55300").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
