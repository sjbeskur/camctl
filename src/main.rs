
use clap::{Parser};
mod cli;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = cli::Config::parse();
    
    camctl::capture(1)?;
    Ok(())
}


