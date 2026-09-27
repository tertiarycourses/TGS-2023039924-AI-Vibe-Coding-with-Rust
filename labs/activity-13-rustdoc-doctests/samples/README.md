# Activity 13 sample scripts

Small, standalone Rust programs that teach doc comments (/// and //!), # Examples sections, doctests and rustdoc. They need only the Rust toolchain (no Cargo project). Open a terminal in this samples/ folder, predict the output, then run the command. On Windows run the program as `.\name.exe`.

## 01_doc_comments.rs

```sh
rustc --edition 2021 --crate-type lib --crate-name docsample 01_doc_comments.rs && rustdoc --edition 2021 --test --crate-type lib --crate-name docsample -L . 01_doc_comments.rs
```

Expected output:

```text
test result: ok. 1 passed
```

## 02_errors_section.rs

```sh
rustc --edition 2021 --crate-type lib --crate-name docsample2 02_errors_section.rs && rustdoc --edition 2021 --test --crate-type lib --crate-name docsample2 -L . 02_errors_section.rs
```

Expected output:

```text
test result: ok. 1 passed
```

Try it: change one value in a sample, predict the new output, run it again. Then ask the AI to explain any surprise.
