# Activity 11: Debug a failing integration with tests

AI Vibe Coding with Rust (TGS-2023039924) - version 3.0 - K4/A4 - about 35 minutes.

## Goal and scenario

The nightly stock summary is wrong: the total is too low and a correctly stocked item is flagged. The starter crate contains two real defects. You reproduce them with tests, diagnose them with the help of an AI assistant, fix them and keep regression tests.

## Exact contract

summarise(csv, threshold) -> Result<Summary, String> where Summary { lines, total_cents, low_stock }. Every line counts towards the total; low stock is strictly below the threshold, sorted by SKU; malformed lines are Err("line N: ...").

## Prerequisites

Rust stable installed with rustup (includes cargo, rustfmt and clippy), Visual Studio Code with rust-analyzer, and an AI coding assistant of your choice. No internet access or external crates are needed. No API keys or paid accounts are required.

## Folder contents

| Path | Purpose |
|---|---|
| `starter/` | The crate you complete. Start here. |
| `solution/` | Reference crate: runs green; compare only after your own attempt. |
| `data/stock.csv` | Input data used by the tests. |
| `PROMPTS.md` / `Vibe-Coding-Prompts.pdf` | The three vibe-coding prompts for this activity. |
| `docs/` | Review and verification records you complete. |

## Step-by-step

1. **Read the specification.** Read the Goal and Exact contract sections below and the files in data/ (if any). Write down each input, its type and valid range, the failure behaviour and the boundary values before you open any code.
2. **Open the activity in VS Code.** Open this activity folder in Visual Studio Code with the rust-analyzer extension enabled. Confirm the toolchain with rustc --version and cargo --version in the integrated terminal.
3. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (Starter: 4 of 6 tests fail. After both fixes cargo test reports 6 passed.)
4. **Run the starter and read the failures.** Run: cd ../starter && cargo test. Four of the six tests fail because the starter contains two real defects; the other two pass. Read the first failure: note the test name, the expected value and the actual value.
5. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
6. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
7. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
8. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
9. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Add a test for threshold 0 (nothing is low) and explain why it passed even before the fix.
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

Starter: 4 of 6 tests fail. After both fixes cargo test reports 6 passed.

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

> Act as a senior Rust reviewer. Specification: summarise(csv, threshold) -> Result<Summary, String> where Summary { lines, total_cents, low_stock }. Every line counts towards the total; low stock is strictly below the threshold, sorted by SKU; malformed lines are Err("line N: ..."). Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Debug a failing integration with tests" in starter/src so that the supplied tests pass. Give the AI the failing test output and the function only; ask for a diagnosis before any code change. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
