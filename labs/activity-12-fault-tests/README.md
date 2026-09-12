# Activity 12 — Build a fault-injection gate

Version2.0 / K4/A4 / 30 minutes / independent practice.

## Goal and scenario

An irrigation request must fail off for stale, disconnected or invalid sensing. Test a guard separately from the pump driver.

## Exact contract

Moisture 0..100 percent; sensor_ok 0/1. Output is ON only below 30 with age<=2000 ms and sensor_ok true; all other cases OFF.

## Prerequisites

A desktop C compiler (cc/clang/gcc), terminal and text editor. Host tests need no Arduino or AI service. Physical extension needs the exact UNO Q, USB cable, reviewed3.3 V-safe peripherals and installed Arduino core; never assume UNO R3 pin/voltage behaviour.

## Folder inventory

Starter and reference .c/.h; assertions; test script; mock CSV/JSON; prompts; documentation templates; adapter where appropriate. No real credentials or .env file is needed.

## Step-by-step

1. Read the specification. Open data/spec.json and data/cases.csv. State each input unit, valid range, failure result and expected boundary before editing. Contract: Moisture 0..100 percent; sensor_ok 0/1. Output is ON only below 30 with age<=2000 ms and sensor_ok true; all other cases OFF.

2. Inspect the shared header. Open starter/logic.h. The declaration must match reference/logic.h exactly; do not change types to silence a warning.

3. Run the supplied reference gate. From this Activity folder run: sh scripts/test.sh reference. It must print the Activity PASS label; this is executed host evidence only.

4. Implement the starter. Open starter/logic.c. Replace each marked TODO return/body with your implementation. Keep the header include and implement every declared function. Use the code examples in the Learner Guide to reason about the contract.

5. Test your implementation. Run: sh scripts/test.sh starter. Investigate the first failed assertion by comparing input, expected result and exact inequality. Do not edit expected values merely to make the test pass.

6. Review with an AI assistant. Read PROMPTS.md. Supply only the mock specification, shared header and your source. Save one proposal, one human acceptance/rejection decision and the test result in docs/review-record.md. No paid API account is required: a trainer can demonstrate review or you can do a manual review.

7. Add a boundary or fault test. Add one independent assertion in tests/test_logic.c. Predict the expected value from the specification first. Re-run both starter and reference to show the test is a contract check rather than a private implementation assumption.

8. Prepare the target wrapper. If adapter/adapter.ino exists, copy the complete adapter sketch folder to your Arduino sketch workspace with logic.c and logic.h from your tested implementation. Select the exact UNO Q board/core. Compile before upload. Keep .ino as C++ and .c as C.

9. Perform board verification only when equipped. Use an LED surrogate only for the compulsory gate demonstration. Before any pump deployment add maximum run time, manual isolation and an approved driver. Disconnect the sensor and verify a measured OFF request.

10. Record evidence and handover. Complete docs/verification-record.md: host compiler and output, sketch compile result, exact variant/core/library versions, wiring review, actual physical result or NOT RUN plus reason. Link each requirement to code and tests; never infer hardware success from PASS.

## Commands

```sh
sh scripts/test.sh reference
sh scripts/test.sh starter
cc --version
```

## Expected result

Seven assertions pass; each injected invalid/stale condition returns OFF.

## Troubleshooting

- Compiler absent: install the platform developer C toolchain, then verify cc --version.
- Header/link error: use exactly matching declarations and the C++ linkage guards.
- Assertion failure: inspect units, boundary comparison and initial state; keep expected values fixed until the contract is corrected.
- Board unavailable: complete host evidence and record physical checks NOT RUN.
- Serial/Bridge difference: verify transport framing and installed versioned APIs; host parsing is not transport proof.

## Deliverables and acceptance

Your implemented starter, test output, added fault/boundary assertion, reviewed AI/manual proposal and requirement-linked verification record. Host reference and starter gates pass; physical claims have actual observed evidence or are explicitly NOT RUN.

## Vibe-coding prompts /same source as slides and PDF

### Design contract

> Design C-compatible logic for this exact contract: Moisture 0..100 percent; sensor_ok 0/1. Output is ON only below 30 with age<=2000 ms and sensor_ok true; all other cases OFF. Keep Arduino APIs in the C++ adapter.

### Vibe-code implementation

> Implement Build a fault-injection gate. Preserve logic.h, units and thresholds. Explain every assumption. Add independently predicted boundary tests; do not invent hardware/API evidence.

### Repair from evidence

> Given the first failed assertion, explain expected versus actual, identify the smallest contract-preserving fix and add a regression test. Do not edit expected values to hide the defect.
