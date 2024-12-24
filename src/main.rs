pub mod commands;
pub mod convert;
pub mod prompt;
pub mod service;

use std::io::Write;
use std::process::{ExitCode, Termination};

use anyhow::Result;
use service::AsyaService;

fn main() -> ExitCode {
    match run() {
        Ok(ret) => ret.report(),
        Err(ref e) => {
            write!(&mut std::io::stderr(), "{}", format_args!("{:?}", e))
                .expect("Error writing to stderr");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    dotenvy::dotenv()?;
    let service = AsyaService::new()?;
    let args: Vec<String> = std::env::args().collect();
    let command = (&args[1..]).join(" ");
    service.execute(command)?;
    Ok(())
}
