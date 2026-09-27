use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (path, threshold) = match args.as_slice() {
        [path] => (path.as_str(), 5),
        [path, threshold] => match threshold.parse() {
            Ok(value) => (path.as_str(), value),
            Err(_) => {
                eprintln!("error: threshold must be a whole number");
                return ExitCode::from(2);
            }
        },
        _ => {
            eprintln!("usage: activity12 <stock.csv> [low-stock-threshold]");
            return ExitCode::from(2);
        }
    };
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("error: cannot read {path}: {error}");
            return ExitCode::from(1);
        }
    };
    match activity12::run(&text, threshold) {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(1)
        }
    }
}
