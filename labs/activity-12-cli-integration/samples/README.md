# Activity 12 sample scripts

Small, standalone Rust programs that teach reading files, command-line arguments, stdout versus stderr and process exit codes. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_read_file_and_exit_code.rs

```sh
rustc --edition 2021 01_read_file_and_exit_code.rs && ./01_read_file_and_exit_code ../data/stock.csv
```

Expected output:

```text
../data/stock.csv: 5 line(s)
```

## 02_stdout_stderr.rs

```sh
rustc --edition 2021 02_stdout_stderr.rs && ./02_stdout_stderr
```

Expected output:

```text
ITEMS 4
exit code will be 0
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
