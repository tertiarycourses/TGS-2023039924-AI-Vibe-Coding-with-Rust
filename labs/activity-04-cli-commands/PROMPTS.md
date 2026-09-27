# Activity 4: prompts

Use these in your AI coding assistant. Paste only the specification, the relevant source file and cargo output; never paste passwords, API keys or assessment answers.

# Part 1: Learn Rust

## Explain it simply

> I am learning Rust. Explain reading command-line arguments, slice patterns, parsing text into numbers and returning Result to a beginner in plain English. Use a short example from a small shop's stock system, keep the code under 20 lines, show its output, and point out one mistake beginners often make.

## Walk me through the sample

> Here is samples/01_command_line_args.rs from my course. Walk through it line by line. Before I run it, ask me to predict the output. Then suggest one small change I can make to experiment, and tell me what should happen.

## Quiz me

> Quiz me on reading command-line arguments, slice patterns, parsing text into numbers and returning Result with 5 questions, one at a time. Wait for my answer each time and explain why it is right or wrong. Include one question where I predict the output or the compiler error of a short Rust snippet.

## Compare with what I know

> I know some Python or JavaScript. In a small table, compare how Rust handles command-line input and parsing with Python and JavaScript. Then show one Rust example and explain what the Rust compiler checks for me that the others do not.

# Part 2: Build with AI (vibe coding)

## Plan before code

> Act as a senior Rust reviewer. Specification: parse_command(&[&str]) -> Result<Command, String>. Accepts exactly: list | add <sku> <qty> | remove <sku> <qty>. qty must parse as u32 and be at least 1. Anything else returns Err with a usage or reason message. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

## Generate with AI

> Implement "Vibe-code a command parser" in starter/src so that the supplied tests pass. Generate the parser from the command grammar only; reject any suggestion that adds crates such as clap. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

## Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.

## Review checklist for every AI proposal

- Public signatures and the specification are unchanged.
- No `unsafe`, no `unwrap()`/`expect()` on input data, no new crates.
- Every boundary in the specification has a test.
- `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` all pass.
- You can explain every line in your own words.
