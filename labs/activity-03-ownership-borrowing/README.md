# Activity 3: Share inventory records with ownership and borrowing

AI Vibe Coding with Rust (TGS-2023039924) - version 3.0 - K1/A1 K3/A3 - about 25 minutes.

## Goal and scenario

Three StockPilot functions handle the same item list differently: one only reads it, one changes one item and one consumes the list. You choose the parameter types that express those intentions.

## Exact contract

Item { sku, name, qty: u32 }. total_quantity(&[Item]) -> u64 reads; restock(&mut Item, u32) -> Option<u32> changes one item and leaves it unchanged on overflow; in_stock(Vec<Item>) -> Vec<Item> consumes the list.

## Prerequisites

Rust stable installed with rustup (includes cargo, rustfmt and clippy), Visual Studio Code with rust-analyzer, and an AI coding assistant of your choice. No internet access or external crates are needed. No API keys or paid accounts are required.

## Folder contents

| Path | Purpose |
|---|---|
| `starter/` | The crate you complete. Start here. |
| `solution/` | Reference crate: runs green; compare only after your own attempt. |
| `PROMPTS.md` / `Vibe-Coding-Prompts.pdf` | The three vibe-coding prompts for this activity. |
| `docs/` | Review and verification records you complete. |

## Step-by-step

1. **Read the specification.** Read the Goal and Exact contract sections below and the files in data/ (if any). Write down each input, its type and valid range, the failure behaviour and the boundary values before you open any code.
2. **Open the activity in VS Code.** Open this activity folder in Visual Studio Code with the rust-analyzer extension enabled. Confirm the toolchain with rustc --version and cargo --version in the integrated terminal.
3. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (cargo test reports 5 passed; the overflow test proves the item is unchanged.)
4. **Run the starter and read the failures.** Run: cd ../starter && cargo test. Every test that calls an unfinished function fails with "not yet implemented" from todo!(). This is expected: the tests are the specification you will implement against.
5. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
6. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
7. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
8. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
9. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Add rename(item: &mut Item, new_name: &str) and a test proving the SKU is unchanged.
10. **Record the evidence.** Complete docs/verification-record.md with the commands you ran, the exact result lines (for example "test result: ok. 5 passed"), the AI proposals you accepted or rejected and why. Compare your code with solution/ only after your own tests pass.

## Commands

```sh
cd solution && cargo test
cd ../starter
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Expected result

cargo test reports 5 passed; the overflow test proves the item is unchanged.

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

> Act as a senior Rust reviewer. Specification: Item { sku, name, qty: u32 }. total_quantity(&[Item]) -> u64 reads; restock(&mut Item, u32) -> Option<u32> changes one item and leaves it unchanged on overflow; in_stock(Vec<Item>) -> Vec<Item> consumes the list. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Share inventory records with ownership and borrowing" in starter/src so that the supplied tests pass. Choose &, &mut or by-value parameters to match each function's intent; avoid clone() unless you justify it. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
