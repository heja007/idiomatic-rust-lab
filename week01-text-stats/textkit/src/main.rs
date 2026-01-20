use clap::{Parser, Subcommand};
use textkit::errors::TextkitError;
use textkit::grep::grep_lines;
use textkit::stats::analyze;
use textkit::uniq::uniq_lines;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Show text statistics for a file.
    Stats {
        #[arg(value_name = "PATH")]
        path: String,
    },
    /// Collapse consecutive duplicate lines, like Unix uniq.
    Uniq {
        #[arg(value_name = "PATH")]
        path: String,
        #[arg(long)]
        all: bool,
    },
    /// Print lines that match the given pattern.
    Grep {
        #[arg(value_name = "PATTERN")]
        pattern: String,
        #[arg(value_name = "PATH")]
        path: String,
        #[arg(short = 'n', long = "line-number")]
        line_number: bool,
        #[arg(short = 'i', long = "ignore-case")]
        ignore_case: bool,
    },
}

fn main() {
    let args = Args::parse();

    let path = match &args.command {
        Command::Stats { path } => path,
        Command::Uniq { path, .. } => path,
        Command::Grep { path, .. } => path,
    };

    let text = match read_text(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    match args.command {
        Command::Stats { .. } => {
            let s = match analyze(&text) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {e}");
                    return;
                }
            };
            println!(
                "lines: {}\nwords: {}\nchars: {}\nbytes: {}",
                s.lines, s.words, s.chars, s.bytes
            );
        }
        Command::Uniq { all, .. } => {
            let lines = match uniq_lines(&text, all) {
                Ok(lines) => lines,
                Err(e) => {
                    eprintln!("Error: {e}");
                    return;
                }
            };
            for line in lines {
                println!("{line}");
            }
        }
        Command::Grep {
            pattern,
            ignore_case,
            line_number,
            ..
        } => {
            let lines = match grep_lines(&text, &pattern, ignore_case, line_number) {
                Ok(lines) => lines,
                Err(e) => {
                    eprintln!("Error: {e}");
                    return;
                }
            };
            for line in lines {
                println!("{line}");
            }
        }
    }
}

fn read_text(path: &str) -> Result<String, TextkitError> {
    Ok(std::fs::read_to_string(path)?)
}
