# Activity 1 sample scripts

Small, standalone Rust programs that teach Cargo, variables, mutability, constants, integer types and functions that return Option. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_variables_and_types.rs

```sh
rustc --edition 2021 01_variables_and_types.rs && ./01_variables_and_types
```

Expected output:

```text
price = 250 cents, qty = 4
after adding one: qty = 5 (max 10000)
total = 1250 cents = $12.50
checked_mul(2) on u64::MAX = None
```

## 02_functions_and_option.rs

```sh
rustc --edition 2021 02_functions_and_option.rs && ./02_functions_and_option
```

Expected output:

```text
4 x 250 = 1000
0 x 250 rejected
10001 x 1 rejected
2 x 18446744073709551615 rejected
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
