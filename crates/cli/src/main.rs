use std::path::PathBuf;

use clap::Parser;
use psarc_lib::archive::PSArchive;

/// psarc-cli List contents of playstation archives
#[derive(Debug, Parser)]
struct Args {
    /// Path of the archive file
    file_path: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Extract manifest
    #[arg(short, long)]
    manifest_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file_contents = std::fs::read(args.file_path)?;
    let archive = PSArchive::parse(&file_contents[..])?;
    println!("archive version: {}", archive.version);
    println!("archive compression: {}", archive.compression);
    if args.verbose {
        println!("{:#?}", archive.table_of_contents);
    }

    if let Some(manifest_path) = args.manifest_path {
        std::fs::write(manifest_path, archive.parse_file(0)?)?;
    }

    Ok(())
}
