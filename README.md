# AI-Assisted C Programming for Arduino

Develop and review embedded C logic, connect it to Arduino C++ sketches, and validate AI-assisted changes with explicit contracts and tests.

[View course details and register](https://www.tertiarycourses.com.sg/wsq-ai-assisted-c-programming-for-arduino.html)

| Course detail | Information |
|---|---|
| Course code | TGS-2023039924 |
| Programme | WSQ |
| Duration | 2 days, 16 hours: 14 training hours and 2 assessment hours |
| TSC | Software Design — ICT-DES-3005-1.1 |
| Courseware | Version 2.0 |
| Funding | Up to 70% for eligible learners. Eligibility and current SSG/provider terms apply. |

## About the course

This course combines C foundations, modular software design, controls, interoperability testing and user-aligned documentation. Arduino UNO Q examples distinguish its Linux application side from deterministic microcontroller logic. Vibe-coding prompts ask for small, reviewable changes; human decisions and independent tests remain essential.

## Learning outcomes

- Determine software components from functional and business specifications.
- Apply software design methodologies and tools in line with organisational practices.
- Select controls, elements and features to meet design objectives.
- Examine component functionality and interoperability.
- Produce detailed design documentation mapped to user specifications.

## Topics covered

- Specifications, C types, functions, compilation and safe GPIO.
- Modular C/C++ boundaries, headers, AI-assisted development and review.
- Control flow, timing, hysteresis, arrays, pointers and bounded strings.
- Transport contracts, integrated records, validation and fault evidence.
- Requirement traceability, maintenance documentation and bounded UNO Q AI proposals.

## Activities

Each folder is self-contained and includes C starter/reference sources, an Arduino C++ adapter, mock data, test scripts, review/evidence templates, prompt text and a **Vibe-Coding-Prompts.pdf**. The `labs/` directory is retained for tool compatibility; all learner-facing work is named Activity.

- [Activity 1: Specify and model a doorbell](labs/activity-01-requirements-doorbell/)
- [Activity 2: Validate a GPIO truth table](labs/activity-02-gpio-truth-table/)
- [Activity 3: Schedule events without delay](labs/activity-03-event-scheduler/)
- [Activity 4: Scale an ADC reading to PWM](labs/activity-04-adc-pwm/)
- [Activity 5: Review modular sensor conversion](labs/activity-05-modular-sensor/)
- [Activity 6: Repair a header and build contract](labs/activity-06-headers-build/)
- [Activity 7: Implement fan modes and hysteresis](labs/activity-07-hysteresis-modes/)
- [Activity 8: Use an array-backed ring buffer](labs/activity-08-ringbuffer-pointers/)
- [Activity 9: Encode bounded status records](labs/activity-09-strings-bitfields/)
- [Activity 10: Parse a bounded serial command](labs/activity-10-serial-contract/)
- [Activity 11: Integrate sensor display and logging](labs/activity-11-module-integration/)
- [Activity 12: Build a fault-injection gate](labs/activity-12-fault-tests/)
- [Activity 13: Document an irrigation interface](labs/activity-13-documentation-capstone/)
- [Activity 14: Bound a UNO Q local-AI suggestion](labs/activity-14-unoq-bounded-ai/)

## Public package and usage

[Editable slide deck](courseware/AI-Assisted-C-Programming-for-Arduino-v2.0.pptx) · [Slide PDF](courseware/AI-Assisted-C-Programming-for-Arduino-v2.0.pdf) · [Learner Guide PDF](courseware/LG-AI-Assisted-C-Programming-for-Arduino-v2.0.pdf) · [Learner Guide Markdown](LG-AI-Assisted-C-Programming-for-Arduino-v2.0.md)

Detailed setup, execution steps, expected results and troubleshooting are in the Learner Guide and activity READMEs. A host C compiler is sufficient for the mock logic tests; UNO Q target work requires the supported Arduino Zephyr board core and RouterBridge library. No paid AI API account is required for the supplied activities.

UNO Q peripherals in this package use 3.3 V-safe signalling. `.c` modules contain C logic; `.ino` adapters compile as C++. A host test or successful target compilation does not prove upload, physical timing, sensor calibration, LCD/SD operation or actuator safety. Record physical checks as NOT RUN until observed. Use LEDs as actuator surrogates; never connect a pump or motor directly to a GPIO or let model text execute arbitrary commands.

## Distribution boundary

This repository contains learner-facing teaching material and activity solutions, not confidential assessment solutions. Assessment papers, answer keys, the trainer Lesson Plan, source/reference material, credentials, private build tooling, archives and QA renders are excluded from GitHub. Assessments are issued through the course LMS; answer keys remain restricted to trainer administration on Drive.

## Sources and acknowledgements

See [Sources](SOURCES.md) for the Arduino UNO Q and C++ references. Board illustrations are attributed to Arduino. AI-generated conceptual imagery is labelled as illustrative, not a pinout or measured project.

Developed by **Tertiary Infotech Academy Pte Ltd**. [Course registration](https://www.tertiarycourses.com.sg/wsq-ai-assisted-c-programming-for-arduino.html).
