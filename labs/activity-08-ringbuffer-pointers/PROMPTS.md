# Activity 8 — bounded review prompts

Use a prose prompt in the assistant UI, not a shell command. No API key is needed.

> Review this C-compatible module against the following exact contract: Buffer capacity is 4; count 0..4; next 0..3; oldest sample overwritten. Empty mean returns 0. Samples are restricted to -1000..1000. Identify one boundary defect, one unit/range risk and one portability concern. Do not change pins, thresholds, units or public signatures. Propose a small patch and independent assertions. Explain assumptions and mark unverified hardware/API claims.

> Generate maintenance notes mapped to requirement IDs, using only the supplied code and executed host evidence. Distinguish host tests, target compile and physical-board measurement. Do not claim upload, LCD, SD or model performance unless the verification record contains observed evidence.

Record proposal, human decision and before/after test outcomes in docs/review-record.md. Use mock data only; do not paste secrets, identities or private assessment answers.

## Design contract

> Design C-compatible logic for this exact contract: Buffer capacity is 4; count 0..4; next 0..3; oldest sample overwritten. Empty mean returns 0. Samples are restricted to -1000..1000. Keep Arduino APIs in the C++ adapter.

## Vibe-code implementation

> Implement Use an array-backed ring buffer. Preserve logic.h, units and thresholds. Explain every assumption. Add independently predicted boundary tests; do not invent hardware/API evidence.

## Repair from evidence

> Given the first failed assertion, explain expected versus actual, identify the smallest contract-preserving fix and add a regression test. Do not edit expected values to hide the defect.
