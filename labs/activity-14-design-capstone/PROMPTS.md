# Activity 14: vibe-coding prompts

Use these in your AI coding assistant. Paste only the specification, the relevant source file and cargo output; never paste passwords, API keys or assessment answers.

## Plan before code

> Act as a senior Rust reviewer. Specification: reorder_suggestions(&[StockItem]) -> Vec<Suggestion>. R1 point = usage x lead + safety; R2 items above the point are excluded; R3 order_qty = point + 7 days usage - on_hand, zero quantities excluded; R4 overflowing items are skipped, never wrapped. Output sorted by SKU. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

## Generate with AI

> Implement "Design and document a reorder feature with AI" in starter/src so that the supplied tests pass. Give the AI R1-R4 and the public types; require a trace table in its answer and verify each row against the tests. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

## Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.

## Review checklist for every AI proposal

- Public signatures and the specification are unchanged.
- No `unsafe`, no `unwrap()`/`expect()` on input data, no new crates.
- Every boundary in the specification has a test.
- `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` all pass.
- You can explain every line in your own words.
