# Activity 12: prompts

Use these in your AI coding assistant. Paste only the specification, the relevant source file and cargo output; never paste passwords, API keys or assessment answers.

# Part 1: Learn Rust

## Explain it simply

> I am learning Rust. Explain reading files, command-line arguments, stdout versus stderr and process exit codes to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

## Walk me through the sample

> Here is samples/01_read_file_and_exit_code.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

## Quiz me

> Quiz me on reading files, command-line arguments, stdout versus stderr and process exit codes with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

## Compare with what I know

> I know some Python or JavaScript. In a small table, compare how Rust handles building command-line tools with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

# Part 2: Build with AI (vibe coding)

## Plan before code

> Act as a senior Rust reviewer. Specification: run(csv, threshold) -> Result<String, String> returns "ITEMS n\nTOTAL $D.CC\nLOW STOCK a, b | none". The binary takes <file> [threshold=5]; exit 0 on success, 1 for unreadable or invalid data, 2 for usage errors. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

## Generate with AI

> Implement "Integrate components into a tested command-line tool" in starter/src so that the supplied tests pass. Keep main thin; put every rule in lib.rs where the library tests can reach it. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

## Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.

## Review checklist for every AI proposal

- Public signatures and the specification are unchanged.
- No `unsafe`, no `unwrap()`/`expect()` on input data, no new crates.
- Every boundary in the specification has a test.
- `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` all pass.
- You can explain every line in your own words.
