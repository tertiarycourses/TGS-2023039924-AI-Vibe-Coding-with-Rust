# Activity 8: vibe-coding prompts

Use these in your AI coding assistant. Paste only the specification, the relevant source file and cargo output; never paste passwords, API keys or assessment answers.

## Plan before code

> Act as a senior Rust reviewer. Specification: Inventory::receive(&mut self, sku, qty) -> Option<u32> (None on overflow, unchanged); ship -> Result<u32, String> (unknown SKU or insufficient stock is Err, unchanged); quantity(sku) -> u32 (0 when unknown); low_stock(threshold) -> Vec<(String, u32)> strictly below threshold, sorted by SKU. Before writing any code, list the public signatures, the edge cases and at least four boundary tests you would expect. Ask me about anything ambiguous.

## Generate with AI

> Implement "Track stock in a HashMap" in starter/src so that the supplied tests pass. Keep the HashMap private and the method signatures unchanged; no unwrap() on lookups. Keep public signatures unchanged, use no unsafe code, and explain each design decision in one sentence.

## Repair from evidence

> Here is my exact cargo output: <paste the first error or failing test>. Explain the cause in plain English, propose the smallest fix that keeps the specification, and add one regression test. Do not change expected values in existing tests.

## Review checklist for every AI proposal

- Public signatures and the specification are unchanged.
- No `unsafe`, no `unwrap()`/`expect()` on input data, no new crates.
- Every boundary in the specification has a test.
- `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` all pass.
- You can explain every line in your own words.
