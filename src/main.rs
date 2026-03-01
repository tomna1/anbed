use std::fs::File;
use std::io::{self, BufRead, IsTerminal};
use std::path::PathBuf;

use clap::Parser;

mod ansi_embed;
use ansi_embed::embed_ansi;

mod error;
pub use error::AnbedError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file to embed ansii escape codes and then print out
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // true if the keyboard is connected as stdin and false if it is connected to a pipe
    let is_tty_connected_terminal = io::stdin().is_terminal();

    match (args.path, is_tty_connected_terminal) {
        (Some(path), _) => {
            let file = File::open(&path).map_err(|_| AnbedError::FileOpen(path.to_path_buf()))?;
            process_lines(io::BufReader::new(file))?;
        }
        (None, false) => {
            let stdin = io::stdin().lock();
            process_lines(io::BufReader::new(stdin))?;
        }
        (None, true) => {
            return Err(AnbedError::BadArguments.into());
        }
    }
    Ok(())
}

fn process_lines<R: BufRead>(reader: R) -> Result<(), AnbedError> {
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let embedded_ansii_text = embed_ansi(&line)?;
                println!("{embedded_ansii_text}");
            }
            Err(err) => {
                return Err(AnbedError::Io(err));
            }
        }
    }
    Ok(())
}
