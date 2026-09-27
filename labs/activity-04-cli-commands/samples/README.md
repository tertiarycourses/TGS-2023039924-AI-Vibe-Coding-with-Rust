# Activity 4 sample scripts

Small, standalone Rust programs that teach reading command-line arguments, slice patterns, parsing text into numbers and returning Result. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_command_line_args.rs

```sh
rustc --edition 2021 01_command_line_args.rs && ./01_command_line_args add ABC-0001 5
```

Expected output:

```text
argument 0: add
argument 1: ABC-0001
argument 2: 5
```

## 02_parse_with_result.rs

```sh
rustc --edition 2021 02_parse_with_result.rs && ./02_parse_with_result
```

Expected output:

```text
    5 -> Ok(5)
    0 -> Err(quantity must be at least 1)
 five -> Err(not a whole number: five)
   -3 -> Err(not a whole number: -3)
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
