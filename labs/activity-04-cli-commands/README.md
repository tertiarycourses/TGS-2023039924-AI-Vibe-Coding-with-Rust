# Activity 4: Vibe-code a command parser

AI Vibe Coding with Rust (TGS-2023039924) - version 3.1 - K2/A2 - about 25 minutes.

## Goal and scenario

Store staff will type commands such as "add ABC-0001 5". You describe the command language to an AI assistant, review the generated parser, and keep only code that passes the acceptance tests.

## Exact contract

parse_command(&[&str]) -> Result<Command, String>. Accepts exactly: list | add <sku> <qty> | remove <sku> <qty>. qty must parse as u32 and be at least 1. Anything else returns Err with a usage or reason message.

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
3. **Learn the concept first.** Open samples/ and run each sample script (commands in samples/README.md). Predict the output before you run it. Then use the "Learn Rust" prompts in PROMPTS.md with your AI assistant to explain reading command-line arguments, slice patterns, parsing text into numbers and returning Result. The Learn more links point to the matching Rust Book and tutorial pages.
4. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (cargo test reports 5 passed; cargo run -- add ABC-0001 5 prints Add { sku: "ABC-0001", qty: 5 }.)
5. **Run the starter and read the failures.** Run: cd ../starter && cargo test. Every test that calls an unfinished function fails with "not yet implemented" from todo!(). This is expected: the tests are the specification you will implement against.
6. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
7. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
8. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
9. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo run -- add ABC-0001 5 ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
10. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Ask the AI to add "count <sku>"; accept it only with a new test for the missing-SKU case.
11. **Record the evidence.** Complete docs/verification-record.md with the commands you ran, the exact result lines (for example "test result: ok. 5 passed"), the AI proposals you accepted or rejected and why. Compare your code with solution/ only after your own tests pass.

## Learn the concepts

This activity uses reading command-line arguments, slice patterns, parsing text into numbers and returning Result. Run the samples, then ask your AI assistant the Learn Rust prompts below.

| Sample | Run it | Expected output |
|---|---|---|
| [`01_command_line_args.rs`](samples/01_command_line_args.rs) | `rustc --edition 2021 01_command_line_args.rs && ./01_command_line_args add ABC-0001 5` | argument 0: add<br>argument 1: ABC-0001<br>argument 2: 5 |
| [`02_parse_with_result.rs`](samples/02_parse_with_result.rs) | `rustc --edition 2021 02_parse_with_result.rs && ./02_parse_with_result` |     5 -> Ok(5)<br>    0 -> Err(quantity must be at least 1)<br> five -> Err(not a whole number: five)<br>   -3 -> Err(not a whole number: -3) |

### Learn Rust prompts

**Explain it simply.** I am learning Rust. Explain reading command-line arguments, slice patterns, parsing text into numbers and returning Result to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

**Walk me through the sample.** Here is samples/01_command_line_args.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

**Quiz me.** Quiz me on reading command-line arguments, slice patterns, parsing text into numbers and returning Result with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

**Compare with what I know.** I know some Python or JavaScript. In a small table, compare how Rust handles command-line input and parsing with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

### Learn more

- [Rust Book: Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [Rust Book: An I/O Project (command-line program)](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [W3Schools: Rust Functions](https://www.w3schools.com/rust/rust_functions.php)
- [Programiz: Rust Function](https://www.programiz.com/rust/function)


## Commands

```sh
cd solution && cargo test
cd ../starter
cargo test
cargo run -- add ABC-0001 5
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Expected result

cargo test reports 5 passed; cargo run -- add ABC-0001 5 prints Add { sku: "ABC-0001", qty: 5 }.

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

> Act as a senior Rust reviewer. Specification: parse_command(&[&str]) -> Result<Command, String>. Accepts exactly: list | add <sku> <qty> | remove <sku> <qty>. qty must parse as u32 and be at least 1. Anything else returns Err with a usage or reason message. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Vibe-code a command parser" in starter/src so that the supplied tests pass. Generate the parser from the command grammar only; reject any suggestion that adds crates such as clap. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
