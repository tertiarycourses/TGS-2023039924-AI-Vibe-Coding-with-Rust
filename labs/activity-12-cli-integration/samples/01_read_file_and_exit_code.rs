// Sample: read a file named on the command line; exit code 1 if it fails.
use std::process::ExitCode;

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "missing.csv".to_string());
    match std::fs::read_to_string(&path) {
        Ok(text) => {
            println!("{path}: {} line(s)", text.lines().count());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: cannot read {path}: {error}");
            ExitCode::from(1)
        }
    }
}
