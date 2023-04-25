use clap::Parser;
mod cli;
use arducam_mipicamera::Encoding;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Config::parse();

    let mut enc = match args.encoding {
        cli::ImageEncoding::JPEG => Encoding::Jpeg,
        _ => Encoding::Png,
    };

    camctl::capture(1, args.mode, args.timeout, enc)?; //, args.encoding)?;
    Ok(())
}
