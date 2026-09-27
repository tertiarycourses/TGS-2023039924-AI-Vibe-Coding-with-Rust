# Activity 8 sample scripts

Small, standalone Rust programs that teach Vec, HashMap, the entry API, Option from get, structs and sorting for deterministic output. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_vec_basics.rs

```sh
rustc --edition 2021 01_vec_basics.rs && ./01_vec_basics
```

Expected output:

```text
sorted: ["AAA-0001", "BBB-0002", "CCC-0003"]
first: Some("AAA-0001"), tenth: None
1. AAA-0001
2. BBB-0002
3. CCC-0003
```

## 02_hashmap_entry.rs

```sh
rustc --edition 2021 02_hashmap_entry.rs && ./02_hashmap_entry
```

Expected output:

```text
[("ABC-0001", 7), ("ABC-0002", 3)]
ZZZ-9999 is not stocked
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
