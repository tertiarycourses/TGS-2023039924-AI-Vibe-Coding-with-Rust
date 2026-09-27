# Activity 13: Document an API with rustdoc and doctests

AI Vibe Coding with Rust (TGS-2023039924) - version 3.1 - K5/A5 - about 30 minutes.

## Goal and scenario

The maintenance team will own the reorder calculation. The starter code works but has no documentation. You write rustdoc comments whose examples are compiled and run as tests, and trace them to requirements.

## Exact contract

reorder_point(daily_usage, lead_days, safety_stock) -> Option<u32> = daily_usage x lead_days + safety_stock (R1), None on overflow (R2). needs_reorder(on_hand, point) is true at or below the point (R3). Every public item is documented; cargo clippy with -D warnings enforces missing_docs.

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
3. **Learn the concept first.** Open samples/ and run each sample script (commands in samples/README.md). Predict the output before you run it. Then use the "Learn Rust" prompts in PROMPTS.md with your AI assistant to explain doc comments (/// and //!), # Examples sections, doctests and rustdoc. The Learn more links point to the matching Rust Book and tutorial pages.
4. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (cargo test runs 3 tests and 3 doctests; cargo doc --no-deps builds; clippy -D warnings is clean.)
5. **Run the starter and read the failures.** Run: cd ../starter && cargo test. cargo test passes because the logic already works, but cargo clippy --all-targets -- -D warnings fails with "missing documentation" errors. Your job is the documentation.
6. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
7. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
8. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
9. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo doc --no-deps ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
10. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Add a # Panics or # Errors section policy to docs/design-note.md and justify why neither applies here.
11. **Record the evidence.** Complete docs/verification-record.md with the commands you ran, the exact result lines (for example "test result: ok. 5 passed"), the AI proposals you accepted or rejected and why. Compare your code with solution/ only after your own tests pass.

## Learn the concepts

This activity uses doc comments (/// and //!), # Examples sections, doctests and rustdoc. Run the samples, then ask your AI assistant the Learn Rust prompts below.

| Sample | Run it | Expected output |
|---|---|---|
| [`01_doc_comments.rs`](samples/01_doc_comments.rs) | `rustc --edition 2021 --crate-type lib --crate-name docsample 01_doc_comments.rs && rustdoc --edition 2021 --test --crate-type lib --crate-name docsample -L . 01_doc_comments.rs` | test result: ok. 1 passed |
| [`02_errors_section.rs`](samples/02_errors_section.rs) | `rustc --edition 2021 --crate-type lib --crate-name docsample2 02_errors_section.rs && rustdoc --edition 2021 --test --crate-type lib --crate-name docsample2 -L . 02_errors_section.rs` | test result: ok. 1 passed |

### Learn Rust prompts

**Explain it simply.** I am learning Rust. Explain doc comments (/// and //!), # Examples sections, doctests and rustdoc to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

**Walk me through the sample.** Here is samples/01_doc_comments.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

**Quiz me.** Quiz me on doc comments (/// and //!), # Examples sections, doctests and rustdoc with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

**Compare with what I know.** I know some Python or JavaScript. In a small table, compare how Rust handles writing documentation with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

### Learn more

- [Rust Book: Publishing a Crate (documentation comments)](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [The rustdoc Book: Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)


## Commands

```sh
cd solution && cargo test
cd ../starter
cargo test
cargo doc --no-deps
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Expected result

cargo test runs 3 tests and 3 doctests; cargo doc --no-deps builds; clippy -D warnings is clean.

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

> Act as a senior Rust reviewer. Specification: reorder_point(daily_usage, lead_days, safety_stock) -> Option<u32> = daily_usage x lead_days + safety_stock (R1), None on overflow (R2). needs_reorder(on_hand, point) is true at or below the point (R3). Every public item is documented; cargo clippy with -D warnings enforces missing_docs. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Document an API with rustdoc and doctests" in starter/src so that the supplied tests pass. Ask the AI to draft docs from the code and requirements only; verify every example by running cargo test. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
