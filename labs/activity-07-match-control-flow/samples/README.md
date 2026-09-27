# Activity 7 sample scripts

Small, standalone Rust programs that teach enums with data, match with guards, if let, ranges in patterns and loops. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_enum_match.rs

```sh
rustc --edition 2021 01_enum_match.rs && ./01_enum_match
```

Expected output:

```text
receive 10
ship 3
ship 250 (large order, needs approval)
adjust by -2
```

## 02_ranges_if_let_loops.rs

```sh
rustc --edition 2021 02_ranges_if_let_loops.rs && ./02_ranges_if_let_loops
```

Expected output:

```text
  0 -> out of stock
  3 -> low
  5 -> ok
 99 -> ok
100 -> bulk
if let found 7
3 2 1 go
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
