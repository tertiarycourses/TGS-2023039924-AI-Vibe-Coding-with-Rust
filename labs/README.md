# AI Vibe Coding with Rust: hands-on activities

StockPilot is the stock service of a small electronics retailer. Every activity builds or reviews one component of it: order lines, product codes, inventory records, commands, reports, data files, reorder planning and the documentation handed to the maintenance team.

Each folder is a self-contained pair of Cargo crates: complete `starter/`, compare with `solution/`. Start every activity with its README.

| # | Activity | Maps to | Time |
|---|---|---|---|
| 1 | [Turn a specification into a Rust function](activity-01-line-total/) | K1/A1 | 25 min |
| 2 | [Validate a product code with strings](activity-02-sku-validation/) | K1/A1 | 25 min |
| 3 | [Share inventory records with ownership and borrowing](activity-03-ownership-borrowing/) | K1/A1 K3/A3 | 25 min |
| 4 | [Vibe-code a command parser](activity-04-cli-commands/) | K2/A2 | 25 min |
| 5 | [Organise a crate into modules](activity-05-modules-visibility/) | K2/A2 | 25 min |
| 6 | [Load inventory JSON with Cargo dependencies](activity-06-cargo-serde-json/) | K2/A2 K4/A4 | 25 min |
| 7 | [Apply stock movements with enums and match](activity-07-match-control-flow/) | K3/A3 | 30 min |
| 8 | [Track stock in a HashMap](activity-08-hashmap-inventory/) | K3/A3 | 30 min |
| 9 | [Price mixed order lines with traits and iterators](activity-09-traits-iterators/) | K3/A3 | 30 min |
| 10 | [Parse CSV records with Result and custom errors](activity-10-error-handling/) | K4/A4 | 30 min |
| 11 | [Debug a failing integration with tests](activity-11-debug-integration/) | K4/A4 | 35 min |
| 12 | [Integrate components into a tested command-line tool](activity-12-cli-integration/) | K4/A4 | 30 min |
| 13 | [Document an API with rustdoc and doctests](activity-13-rustdoc-doctests/) | K5/A5 | 30 min |
| 14 | [Design and document a reorder feature with AI](activity-14-design-capstone/) | K5/A5 K4/A4 | 30 min |

## Setup once

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # macOS/Linux
# Windows: download and run rustup-init.exe from https://rustup.rs
rustup component add clippy rustfmt
rustc --version && cargo --version
```

Install Visual Studio Code and the rust-analyzer extension. Any AI coding assistant can be used; no API key is needed for the activities.

## Learn Rust: recommended resources

- [Rust official learning hub (the Book, Rustlings, Rust by Example)](https://rust-lang.org/learn/)
- [W3Schools Rust tutorial](https://www.w3schools.com/rust/)
- [W3Schools: Get Started with Rust](https://www.w3schools.com/rust/rust_getstarted.php)
- [Programiz: Learn Rust](https://www.programiz.com/rust)
- [IONOS Digital Guide: Rust tutorial](https://www.ionos.com/digitalguide/websites/web-development/rust-tutorial/)
- [JetBrains: Getting Started with Rust](https://lp.jetbrains.com/getting-started-with-rust/)
- [OneCompiler: Rust tutorial (runs in the browser)](https://onecompiler.com/tutorials/rust)
- [It's FOSS: Rust programming tutorial series](https://itsfoss.com/rust-tutorials/)
- [r/rust community thread: "Rust tutorial that actually teaches Rust"](https://www.reddit.com/r/rust/comments/15b9rl5/rust_tutorial_that_actually_teaches_rust/)
