# Activity 9 sample scripts

Small, standalone Rust programs that teach traits, default methods, generics, trait objects (dyn), closures and iterator adaptors. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_trait_basics.rs

```sh
rustc --edition 2021 01_trait_basics.rs && ./01_trait_basics
```

Expected output:

```text
cable: 1798 cents
setup: 10000 cents
mixed order total: 11798 cents
```

## 02_closures_iterators.rs

```sh
rustc --edition 2021 02_closures_iterators.rs && ./02_closures_iterators
```

Expected output:

```text
at least 1000: [2499, 1299]
doubled: [1798, 4998, 2598, 700]
checked total: Some(5047)
899 with 9% GST: 979
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
