use anyhow::Result;
use axum::{Router, http::StatusCode, routing};
use flexi_logger::{Logger, writers::LogWriter};
use log::{error, info, warn};
use std::process::Command as StdCommand;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    process::Command,
};

struct OpLogWriter;

impl LogWriter for OpLogWriter {
    fn write(
        &self,
        _now: &mut flexi_logger::DeferredNow,
        record: &log::Record,
    ) -> std::io::Result<()> {
        let content = format!(
            "{} [{}] {}",
            record.level(),
            record.module_path().unwrap_or("<unnamed>"),
            record.args()
        );
        // TODO: use tokio
        StdCommand::new("logger").arg(content).status()?;
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }
}

async fn write_to_file(output: &str) -> Result<()> {
    let mut file = File::create("/tmp/ospf-info.txt").await?;
    file.write_all(output.as_bytes()).await?;
    Ok(())
}

async fn read_from_file() -> Result<String> {
    let mut file = File::open("/tmp/ospf-info.txt").await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

async fn get_birdc_output() -> Result<String> {
    let output = Command::new("birdc")
        .arg("s")
        .arg("o")
        .arg("s")
        .output()
        .await?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    write_to_file(&stdout).await?;
    Ok(stdout)
}

async fn ospf_info() -> (StatusCode, String) {
    match get_birdc_output().await {
        Ok(output) => (StatusCode::OK, output),
        Err(e) => {
            warn!("failed to get birdc output: {}", e);
            match read_from_file()
                .await
                .inspect_err(|e| error!("failed to open file: {}", e))
            {
                Ok(content) => (StatusCode::OK, content),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_str("info")?
        .log_to_writer(Box::new(OpLogWriter))
        .start()?;
    info!("ospf info service started");
    let app = Router::new().route("/get/ospf-info", routing::get(ospf_info));
    let listener = TcpListener::bind("0.0.0.0:55300").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
