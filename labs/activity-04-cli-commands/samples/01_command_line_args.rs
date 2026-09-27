// Sample: reading command-line arguments.
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("no arguments: try ./01_command_line_args add ABC-0001 5");
        return;
    }
    for (index, arg) in args.iter().enumerate() {
        println!("argument {index}: {arg}");
    }
}
