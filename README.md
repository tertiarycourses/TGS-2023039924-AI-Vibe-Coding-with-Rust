# AI Vibe Coding with Rust

Build reliable Rust applications with an AI coding assistant, using the compiler, tests, clippy and rustfmt as the gate for every AI-generated change.

| Course detail | Information |
|---|---|
| Course code | `TGS-2023039924` |
| Programme | WSQ |
| Duration | 2 days, 16 hours (14 training hours + 2 assessment hours), 9:30 AM - 6:30 PM |
| Registration | [View course details and register](https://www.tertiarycourses.com.sg/wsq-ai-vibe-coding-with-rust.html) |
| Skills framework | Software Design, ICT-DES-3005-1.1 |
| Funding | Up to 70% SkillsFuture funding for eligible learners. Eligibility and terms apply. |
| Courseware | Version 3.0 |

## About the course

This WSQ course teaches practical Rust programming together with AI-assisted "vibe coding". You turn functional requirements into Rust components through natural-language prompts, then prove every AI draft with the compiler, unit and integration tests, clippy and rustfmt before accepting it.

All 14 activities build parts of **StockPilot**, a stock service for a small electronics retailer: order pricing, product-code validation, inventory records, command parsing, JSON data, error handling, debugging, a command-line tool, and documented reorder planning.

## Learning outcomes

- LO1: Determine basic software components using programming methodologies to meet functional specifications.
- LO2: Apply programming methodologies and tools for software creation.
- LO3: Select essential programming controls and features to meet software design requirements.
- LO4: Examine the interoperability and functionality of programming components.
- LO5: Generate programming design documentation aligned with user specifications.

## Topics covered

1. Rust Programming Fundamentals and Software Components
2. AI Vibe Coding for Rust Application Development
3. Rust Programming Controls, Functions and Features
4. Testing, Debugging and Integrating Rust Components
5. AI-Assisted Rust Software Design and Documentation

## Activities

Each activity folder contains a `starter/` crate to complete, a `solution/` crate for comparison, data files, the vibe-coding prompts (`PROMPTS.md` and `Vibe-Coding-Prompts.pdf`) and review/verification record templates. Start with the [activities index](labs/README.md).

| # | Activity | Maps to |
|---|---|---|
| 1 | [Turn a specification into a Rust function](labs/activity-01-line-total/) | K1/A1 |
| 2 | [Validate a product code with strings](labs/activity-02-sku-validation/) | K1/A1 |
| 3 | [Share inventory records with ownership and borrowing](labs/activity-03-ownership-borrowing/) | K1/A1 K3/A3 |
| 4 | [Vibe-code a command parser](labs/activity-04-cli-commands/) | K2/A2 |
| 5 | [Organise a crate into modules](labs/activity-05-modules-visibility/) | K2/A2 |
| 6 | [Load inventory JSON with Cargo dependencies](labs/activity-06-cargo-serde-json/) | K2/A2 K4/A4 |
| 7 | [Apply stock movements with enums and match](labs/activity-07-match-control-flow/) | K3/A3 |
| 8 | [Track stock in a HashMap](labs/activity-08-hashmap-inventory/) | K3/A3 |
| 9 | [Price mixed order lines with traits and iterators](labs/activity-09-traits-iterators/) | K3/A3 |
| 10 | [Parse CSV records with Result and custom errors](labs/activity-10-error-handling/) | K4/A4 |
| 11 | [Debug a failing integration with tests](labs/activity-11-debug-integration/) | K4/A4 |
| 12 | [Integrate components into a tested command-line tool](labs/activity-12-cli-integration/) | K4/A4 |
| 13 | [Document an API with rustdoc and doctests](labs/activity-13-rustdoc-doctests/) | K5/A5 |
| 14 | [Design and document a reorder feature with AI](labs/activity-14-design-capstone/) | K5/A5 K4/A4 |

Every solution crate passes `cargo test` (including doctests), `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` on Rust 1.93.1. Every starter compiles and fails in the intended way until you complete it.

## Getting started

```sh
# 1. Install Rust (Windows: run rustup-init.exe from https://rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt

# 2. Get the activities and check the toolchain
git clone https://github.com/tertiarycourses/TGS-2023039924-AI-Vibe-Coding-with-Rust.git
cd TGS-2023039924-AI-Vibe-Coding-with-Rust/labs/activity-01-line-total/solution
cargo test
```

Use Visual Studio Code with the rust-analyzer extension and any AI coding assistant. No API key or paid account is needed. Detailed step-by-step instructions for every activity are in the Learner Guide and each activity README.

## Courseware package

| Item | Files |
|---|---|
| Slide deck | [PowerPoint](courseware/AI-Vibe-Coding-with-Rust-v3.0.pptx) · [PDF](courseware/AI-Vibe-Coding-with-Rust-v3.0.pdf) |
| Learner Guide | [Word](courseware/LG-AI-Vibe-Coding-with-Rust-v3.0.docx) · [PDF](courseware/LG-AI-Vibe-Coding-with-Rust-v3.0.pdf) · [Markdown](LG-AI-Vibe-Coding-with-Rust-v3.0.md) |
| Lesson Plan | [Word](courseware/LP-AI-Vibe-Coding-with-Rust-v3.0.docx) · [PDF](courseware/LP-AI-Vibe-Coding-with-Rust-v3.0.pdf) |
| Activities | [labs/](labs/) |

## Distribution boundary

This repository contains learner-facing teaching material: slides, Learner Guide, Lesson Plan and the activity crates with their solutions. Assessment papers, answer keys, reference sources, the assessment generator, credentials and archived versions are not published here. Assessment question papers are issued through the course LMS; answer keys remain restricted to trainers.

## Sources

See [Sources](SOURCES.md) for the official Rust documentation used by this course.

Developed by **Tertiary Infotech Academy Pte Ltd** (UEN 201200696W). [Course registration](https://www.tertiarycourses.com.sg/wsq-ai-vibe-coding-with-rust.html)
