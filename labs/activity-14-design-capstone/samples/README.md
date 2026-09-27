# Activity 14 sample scripts

Small, standalone Rust programs that teach designing with types (newtypes), requirement-named tests and design trade-offs. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_design_with_types.rs

```sh
rustc --edition 2021 01_design_with_types.rs && ./01_design_with_types
```

Expected output:

```text
order 90 x ABC-0001
bad SKU accepted? false
```

## 02_trace_with_tests.rs

```sh
rustc --edition 2021 --test 02_trace_with_tests.rs && ./02_trace_with_tests
```

Expected output:

```text
test result: ok. 3 passed
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
