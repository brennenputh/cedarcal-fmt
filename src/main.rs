use std::{fs::read_to_string, path::PathBuf, process::ExitCode};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    input_file: String,

    /// Output file.  Default: ./output.ics
    #[arg(short, long)]
    output_file: Option<String>
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let input_filename = cli.input_file;
    let output_filename = match cli.output_file {
        Some(f) => f,
        None => "./output.ics".to_string(),
    };

    let input_contents = match read_to_string(input_filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not access file: {e}");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
