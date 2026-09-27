# Activity 9: Price mixed order lines with traits and iterators

AI Vibe Coding with Rust (TGS-2023039924) - version 3.1 - K3/A3 - about 30 minutes.

## Goal and scenario

Orders can contain products and services (for example, device set-up hours). A Priced trait gives both one pricing contract, and iterator chains total and filter them without manual index loops.

## Exact contract

trait Priced { unit_cents, quantity, line_cents (default, checked) }. order_total<T: Priced>(&[T]) and mixed_total(&[&dyn Priced]) return Option<u64> (None on overflow; empty is Some(0)). names_at_least(&[Product], min_cents) -> Vec<&str> in input order.

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
3. **Learn the concept first.** Open samples/ and run each sample script (commands in samples/README.md). Predict the output before you run it. Then use the "Learn Rust" prompts in PROMPTS.md with your AI assistant to explain traits, default methods, generics, trait objects (dyn), closures and iterator adaptors. The Learn more links point to the matching Rust Book and tutorial pages.
4. **Run the reference solution.** In the terminal run: cd solution && cargo test. Confirm the result line shows every test passing. This proves your toolchain works and shows the target behaviour. (cargo test reports 5 passed; the mixed order totals 14,297 cents.)
5. **Run the starter and read the failures.** Run: cd ../starter && cargo test. Every test that calls an unfinished function fails with "not yet implemented" from todo!(). This is expected: the tests are the specification you will implement against.
6. **Plan with the AI assistant.** Paste the "Plan before code" prompt from PROMPTS.md into your AI coding assistant (for example GitHub Copilot Chat, Claude or ChatGPT). Compare its edge cases with your own list from step 1 and record agreements and disagreements in docs/review-record.md.
7. **Vibe-code the implementation.** Use the "Generate with AI" prompt to generate code for starter/src/lib.rs. Review the proposal before accepting it: public signatures unchanged, no unsafe, no unwrap() or expect() on input data, no new crates, and every line explained. Reject or edit anything you cannot justify.
8. **Test until green.** Run cargo test after each accepted change. If a test fails or the code does not compile, use the "Repair from evidence" prompt with the exact cargo output. Never edit expected values in tests to make them pass.
9. **Apply the quality gates.** Run every gate from the starter folder: cargo test ; cargo clippy --all-targets -- -D warnings ; cargo fmt --check. Fix each clippy warning and formatting difference; these are the organisation's coding standards.
10. **Add your own boundary test.** Predict one more boundary case from the specification and add it to the test file in starter/tests/. Run it against both starter and solution. Stretch: Add a Subscription type implementing Priced and include it in mixed_total without changing that function.
11. **Record the evidence.** Complete docs/verification-record.md with the commands you ran, the exact result lines (for example "test result: ok. 5 passed"), the AI proposals you accepted or rejected and why. Compare your code with solution/ only after your own tests pass.

## Learn the concepts

This activity uses traits, default methods, generics, trait objects (dyn), closures and iterator adaptors. Run the samples, then ask your AI assistant the Learn Rust prompts below.

| Sample | Run it | Expected output |
|---|---|---|
| [`01_trait_basics.rs`](samples/01_trait_basics.rs) | `rustc --edition 2021 01_trait_basics.rs && ./01_trait_basics` | cable: 1798 cents<br>setup: 10000 cents<br>mixed order total: 11798 cents |
| [`02_closures_iterators.rs`](samples/02_closures_iterators.rs) | `rustc --edition 2021 02_closures_iterators.rs && ./02_closures_iterators` | at least 1000: [2499, 1299]<br>doubled: [1798, 4998, 2598, 700]<br>checked total: Some(5047)<br>899 with 9% GST: 979 |

### Learn Rust prompts

**Explain it simply.** I am learning Rust. Explain traits, default methods, generics, trait objects (dyn), closures and iterator adaptors to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

**Walk me through the sample.** Here is samples/01_trait_basics.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

**Quiz me.** Quiz me on traits, default methods, generics, trait objects (dyn), closures and iterator adaptors with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

**Compare with what I know.** I know some Python or JavaScript. In a small table, compare how Rust handles traits, generics and iterators with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

### Learn more

- [Rust Book: Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust Book: Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Programiz: Rust Trait](https://www.programiz.com/rust/trait)
- [Programiz: Rust Generics](https://www.programiz.com/rust/generics)
- [Programiz: Rust Closure](https://www.programiz.com/rust/closure)
- [Programiz: Rust Iterators](https://www.programiz.com/rust/iterators)


## Commands

```sh
cd solution && cargo test
cd ../starter
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Expected result

cargo test reports 5 passed; the mixed order totals 14,297 cents.

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

> Act as a senior Rust reviewer. Specification: trait Priced { unit_cents, quantity, line_cents (default, checked) }. order_total<T: Priced>(&[T]) and mixed_total(&[&dyn Priced]) return Option<u64> (None on overflow; empty is Some(0)). names_at_least(&[Product], min_cents) -> Vec<&str> in input order. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

### Generate with AI

> Implement "Price mixed order lines with traits and iterators" in starter/src so that the supplied tests pass. Implement with iterator adaptors (iter, filter, map, try_fold); no index-based for loops. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

### Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.
