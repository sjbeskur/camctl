use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author="Sam Beskur <sam.beskur@gmail.com>", version, about="Arducam Controller", long_about = "Test utility for Arducam controller.")]
pub struct Config{
    
    #[arg(short = 'm', long = "mode", default_value_t = 9 )]
    pub mode: i32,

    #[arg(short = 't', long = "timeout", default_value_t = 2000 )]
    pub timeout: i32,

    //#[arg(short = 'e', long = "encoding", default_value_t = ImageEncoding::JPEG )]
    pub encoding: ImageEncoding,
    
    #[arg(short = 'c', long = "connectivity", default_value_t = 4 )]
    pub connectivity: u8,  // TODO: figure out how to only allow 4 or 8?
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ImageEncoding{
    JPEG,
    PNG,
}