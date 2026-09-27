# Activity 2 sample scripts

Small, standalone Rust programs that teach String versus &str, trimming, UTF-8 bytes versus chars, and validating text. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_string_and_str.rs

```sh
rustc --edition 2021 01_string_and_str.rs && ./01_string_and_str
```

Expected output:

```text
"  abc-1234  ": 12 bytes, 12 chars
"abc-1234": 8 bytes, 8 chars
"ABC-1234": 8 bytes, 8 chars
"ÄBC-1234": 9 bytes, 8 chars
```

## 02_chars_and_validation.rs

```sh
rustc --edition 2021 02_chars_and_validation.rs && ./02_chars_and_validation
```

Expected output:

```text
"ABC-1234" valid = true
" ABC-1234\n" valid = true
"abc-1234" valid = false
"ABC-12A4" valid = false
"ÄBC-1234" valid = false
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
