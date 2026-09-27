# Activity 10 sample scripts

Small, standalone Rust programs that teach Result, the ? operator, custom error enums, Display and Box<dyn Error>. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_result_and_question_mark.rs

```sh
rustc --edition 2021 01_result_and_question_mark.rs && ./01_result_and_question_mark
```

Expected output:

```text
Ok(10)
error: invalid digit found in string
```

## 02_custom_error.rs

```sh
rustc --edition 2021 02_custom_error.rs && ./02_custom_error
```

Expected output:

```text
left: 3
error: unknown SKU ZZZ-9999
error: requested 9 but only 5 available
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
