# Activity 5 — bounded review prompts

Use a prose prompt in the assistant UI, not a shell command. No API key is needed.

> Review this C-compatible module against the following exact contract: Maximum 1..65535; clamped raw input; rounded integer percentage 0..100. No Serial, GPIO or allocation in this module. Identify one boundary defect, one unit/range risk and one portability concern. Do not change pins, thresholds, units or public signatures. Propose a small patch and independent assertions. Explain assumptions and mark unverified hardware/API claims.

> Generate maintenance notes mapped to requirement IDs, using only the supplied code and executed host evidence. Distinguish host tests, target compile and physical-board measurement. Do not claim upload, LCD, SD or model performance unless the verification record contains observed evidence.

Record proposal, human decision and before/after test outcomes in docs/review-record.md. Use mock data only; do not paste secrets, identities or private assessment answers.

## Design contract

> Design C-compatible logic for this exact contract: Maximum 1..65535; clamped raw input; rounded integer percentage 0..100. No Serial, GPIO or allocation in this module. Keep Arduino APIs in the C++ adapter.

## Vibe-code implementation

> Implement Review modular sensor conversion. Preserve logic.h, units and thresholds. Explain every assumption. Add independently predicted boundary tests; do not invent hardware/API evidence.

## Repair from evidence

> Given the first failed assertion, explain expected versus actual, identify the smallest contract-preserving fix and add a regression test. Do not edit expected values to hide the defect.
