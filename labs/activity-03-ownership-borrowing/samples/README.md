# Activity 3 sample scripts

Small, standalone Rust programs that teach ownership, moves, Copy types, clone, shared borrows (&) and mutable borrows (&mut). They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_move_and_clone.rs

```sh
rustc --edition 2021 01_move_and_clone.rs && ./01_move_and_clone
```

Expected output:

```text
consumed 2 items; the clone still has ["cable", "charger"]
n = 5, m = 5
```

## 02_borrowing.rs

```sh
rustc --edition 2021 02_borrowing.rs && ./02_borrowing
```

Expected output:

```text
total before = 12
stock after  = [5, 10, 7]
total after  = 22
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
