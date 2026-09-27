# Activity 5 sample scripts

Small, standalone Rust programs that teach modules, pub visibility, paths with crate::, self:: and super::, and use ... as. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_inline_modules.rs

```sh
rustc --edition 2021 01_inline_modules.rs && ./01_inline_modules
```

Expected output:

```text
Cable x3 @ $2.50 = $7.50
private: only code inside `money` can call this
```

## 02_nested_paths.rs

```sh
rustc --edition 2021 02_nested_paths.rs && ./02_nested_paths
```

Expected output:

```text
stock count = 12
via use-as: 12
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
