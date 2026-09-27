// Sample: results to stdout, diagnostics to stderr, and an explicit exit code.
fn main() {
    println!("ITEMS 4"); // stdout: the report, safe to pipe into a file
    eprintln!("note: diagnostics go to stderr"); // stderr: messages for people
    let code = if std::env::args().count() > 1 { 2 } else { 0 };
    println!("exit code will be {code}");
    std::process::exit(code);
}
