# Course sources

Course identity, duration and outcomes: [Official course page](https://www.tertiarycourses.com.sg/wsq-ai-assisted-c-programming-for-arduino.html).

## Arduino UNO Q

- [UNO Q documentation](https://docs.arduino.cc/hardware/uno-q/).
- [UNO Q product overview](https://www.arduino.cc/product-uno-q).
- [Qualcomm developer introduction](https://www.qualcomm.com/developer/blog/2025/10/meet-arduino-uno-q-qualcomm-dragonwing-fueled-ai-in-a-blink).
- [Official datasheet and user manual](https://docs.arduino.cc/resources/datasheets/ABX00162-datasheet.pdf). The 17 June 2026 revision governs electrical limits in this release.
- [Electromaker feature overview](https://www.electromaker.io/blog/article/arduino-uno-q-specs-features-and-why-qualcomms-ai-partnership-matters) — supplementary context, not the authority for electrical limits.
- [Beyond edge AI: local intelligence](https://blog.arduino.cc/2026/06/05/beyond-edge-ai-bringing-local-intelligence-to-arduino-uno-q/).
- [Local AI agents on UNO Q](https://blog.arduino.cc/2026/06/09/local-ai-agents-on-arduino-uno-q/) — experimental extensions, not a performance or safety guarantee.
- [Official store overview](https://store-usa.arduino.cc/pages/uno-q).
- [Arduino UNO Q Projects for Beginners](https://books.google.com/books/about/Arduino_UNO_Q_Projects_for_Beginners.html?id=lgYX0gEACAAJ) — bibliographic reference only; no full book text was available for this course build.

## C and C++

- [Google C++ Education](https://developers.google.com/edu/c++).
- [Google C++ Getting Started](https://developers.google.com/edu/c++/getting-started) — decomposition, readable examples, input handling, scope and file-processing context.
- [Learn C++ interactive tutorials](https://www.learn-cpp.org/).
- [LearnCpp tutorials](https://www.learncpp.com/).
- [Arduino sketch build process](https://docs.arduino.cc/arduino-cli/sketch-build-process) — `.ino` preprocessing and C/C++ linkage. AVR-specific output details are not applied to UNO Q.
- [Arduino sketch specification](https://docs.arduino.cc/arduino-cli/sketch-specification/).

## Verified runtime contracts

- [ArduinoCore-zephyr](https://github.com/arduino/ArduinoCore-zephyr/tree/79b3f1afdad455f55e4a25030953617152c0227c) — source inspection; target compile validation uses Arduino Q Boards 1.0.0.
- [Arduino_RouterBridge](https://github.com/arduino-libraries/Arduino_RouterBridge/tree/b8ec5b4b92f478981278c82bda0b3338505d16ba) — MCU RPC, callback safety and monitoring.
- [Arduino App Utils Bridge](https://github.com/arduino/app-bricks-py/blob/6a3c2088ef8c319d8f74620716f0f6c187f36cba/src/arduino/app_utils/bridge.py) — Python calls, explicit timeout and error handling.

Arduino documentation board artwork is used with attribution to Arduino and its source documentation. Consult the licensing notices in the original documentation; attribution does not replace current electrical guidance. New teaching examples and diagrams are course-authored. AI-generated illustrations are conceptual and must not be used as wiring instructions.
