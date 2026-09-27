# Activity 6 sample scripts

Small, standalone Rust programs that teach Cargo dependencies, Cargo.lock, parsing structured text and what serde automates. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_manual_parsing.rs

```sh
rustc --edition 2021 01_manual_parsing.rs && ./01_manual_parsing
```

Expected output:

```text
"ABC-0001, 40" -> sku=ABC-0001 qty=40
"ABC-0002,-1" -> error: qty: invalid digit found in string
"no comma" -> error: expected sku,qty
```

## 02_serialize_by_hand.rs

```sh
rustc --edition 2021 02_serialize_by_hand.rs && ./02_serialize_by_hand
```

Expected output:

```text
{"sku":"ABC-0001","name":"USB-C cable","qty":40,"unit_cents":899}
Hand-written JSON breaks if a name contains a quote; serde escapes it for you.
```

## solution/examples/json_demo.rs (uses the serde crates)

```sh
cd ../solution
cargo run --example json_demo
```

The first line of output is: `loaded 1 item(s); value = Some(35960) cents`

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
