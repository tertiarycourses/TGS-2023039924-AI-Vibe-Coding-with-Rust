# Activity 11 sample scripts

Small, standalone Rust programs that teach assertions, dbg!, unit tests with #[test], and reading a failing test. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_dbg_and_assert.rs

```sh
rustc --edition 2021 01_dbg_and_assert.rs && ./01_dbg_and_assert
```

Expected output:

```text
correct total 52655, total without the first line 16695
difference 35960 = the first line
```

## 02_unit_tests.rs

```sh
rustc --edition 2021 --test 02_unit_tests.rs && ./02_unit_tests
```

Expected output:

```text
test result: ok. 3 passed
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
