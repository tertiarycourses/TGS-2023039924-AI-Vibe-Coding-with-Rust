# Activity 1: Turn a specification into a Rust function

AI Vibe Coding with Rust (TGS-2023039924) - version 3.1 - K1/A1 - about 25 minutes.

## Goal and scenario

StockPilot must price an order line. The specification limits quantity to 1..10,000 and forbids a silently wrapped total. You create the Cargo package, write the function and prove it with tests.

## Exact contract

line_total(unit_price_cents: u64, qty: u32) -> Option<u64>. qty 0 or above MAX_QTY (10,000) returns None; an overflowing product returns None; otherwise Some(total cents).

## Prerequisites

Rust stable installed with rustup (includes cargo, rustfmt and clippy), Visual Studio Code with rust-analyzer, and an AI coding assistant of your choice. No internet access or external crates are needed. No API keys or paid accounts are required.

## Folder contents

| Path | Purpose |
|---|---|
| `starter/` | The crate you complete. Start here. |
| `solution/` | Reference crate: runs green; compare only after your own attempt. |
| `samples/` | Small runnable Rust sample scripts that teach the concepts of this activity. |
| `PROMPTS.md` / `Vibe-Coding-Prompts.pdf` | Learn Rust prompts, vibe-coding prompts, samples and learn-more links. |
| `docs/` | Review and verification records you complete. |

## Step-by-step

1. **Read the specification.** Read the Goal and Exact contract sections below and the files in data/ (if any). Write down each input, its type and valid range, the failure behaviour and the boundary values before you open any code.
2. **Open the activity in VS Code.** Open this activity folder in Visual Studio Code with the rust-analyzer extension enabled. Confirm the toolchain with rustc --version and cargo --version in the integrated terminal.
3. **Learn the concept first.** Open samples/ and run each sample script (commands in samples/README.md). Predict the output before you run it. Then use the "Learn Rust" prompts in PROMPTS.md with your AI assistant to explain Cargo, variables, mutability, constants, integer types and functions that return Option. The Learn more links point to the matching Rust Book and tutorial pages.
4. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (cargo test reports 5 passed; cargo run prints "4 x 250 cents = 1000 cents".)
5. **Run the starter and read the failures.** Run: cd ../starter && cargo test. Every test that calls an unfinished function fails with "not yet implemented" from todo!(). This is expected: the tests are the specification you will implement against.
6. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
7. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
8. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
9. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo run ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
10. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Add a discount_percent parameter (0..=100) and two boundary tests for 0 and 100.
11. **Record the evidence.** Complete docs/verification-record.md with the commands you ran, the exact result lines (for example "test result: ok. 5 passed"), the AI proposals you accepted or rejected and why. Compare your code with solution/ only after your own tests pass.

## Learn the concepts

This activity uses Cargo, variables, mutability, constants, integer types and functions that return Option. Run the samples, then ask your AI assistant the Learn Rust prompts below.

| Sample | Run it | Expected output |
|---|---|---|
| [`01_variables_and_types.rs`](samples/01_variables_and_types.rs) | `rustc --edition 2021 01_variables_and_types.rs && ./01_variables_and_types` | price = 250 cents, qty = 4<br>after adding one: qty = 5 (max 10000)<br>total = 1250 cents = $12.50<br>checked_mul(2) on u64::MAX = None |
| [`02_functions_and_option.rs`](samples/02_functions_and_option.rs) | `rustc --edition 2021 02_functions_and_option.rs && ./02_functions_and_option` | 4 x 250 = 1000<br>0 x 250 rejected<br>10001 x 1 rejected<br>2 x 18446744073709551615 rejected |

### Learn Rust prompts

**Explain it simply.** I am learning Rust. Explain Cargo, variables, mutability, constants, integer types and functions that return Option to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

**Walk me through the sample.** Here is samples/01_variables_and_types.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

**Quiz me.** Quiz me on Cargo, variables, mutability, constants, integer types and functions that return Option with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

**Compare with what I know.** I know some Python or JavaScript. In a small table, compare how Rust handles variables, integer types and overflow with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

### Learn more

- [Rust Book: Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [W3Schools: Rust Variables](https://www.w3schools.com/rust/rust_variables.php)
- [W3Schools: Rust Data Types](https://www.w3schools.com/rust/rust_data_types.php)
- [Programiz: Variables and Mutability](https://www.programiz.com/rust/variables-mutability)
- [Programiz: Cargo](https://www.programiz.com/rust/cargo)


## Commands

```sh
cd solution && cargo test
cd ../starter
cargo test
cargo run
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Expected result

cargo test reports 5 passed; cargo run prints "4 x 250 cents = 1000 cents".

## Troubleshooting

| Symptom | What to do |
|---|---|
| `cargo: command not found` | Install Rust from https://rustup.rs, then open a new terminal. |
| `not yet implemented` panic | Expected in the starter: implement the function named in the message. |
| Borrow-checker error (E0382, E0499, E0502) | Read the compiler note; decide who owns the value and whether a `&` or `&mut` borrow is enough. |
| clippy warning | Read the lint link in the message; apply the suggested idiom or explain why not in the review record. |
| AI suggests a new crate or `unwrap()` | Reject it: the contract forbids both. Ask for a std-only, error-returning version. |

## Deliverables

Your completed `starter/` crate passing every gate, one extra boundary test, `docs/review-record.md` and `docs/verification-record.md`.

## Vibe-coding prompts

### Plan before code

> Act as a senior Rust reviewer. Specification: line_total(unit_price_cents: u64, qty: u32) -> Option<u64>. qty 0 or above MAX_QTY (10,000) returns None; an overflowing product returns None; otherwise Some(total cents). Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Turn a specification into a Rust function" in starter/src so that the supplied tests pass. Keep the u64/u32 types and the Option return; use checked arithmetic only. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
