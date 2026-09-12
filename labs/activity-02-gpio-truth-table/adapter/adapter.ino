// Arduino C++ wrapper around C-compatible logic; LED surrogate only.
// Verify selected UNO Q core/pinout. Do not connect motors or pumps directly.
#include "logic.h"
#include <Arduino_RouterBridge.h>
const int BUTTON_PIN=2;
const int ENABLE_PIN=4; // Switch to GND enables; reviewed3.3V input.
void setup() {
 pinMode(LED_BUILTIN,OUTPUT); pinMode(BUTTON_PIN,INPUT_PULLUP);
 pinMode(ENABLE_PIN,INPUT_PULLUP);
 Bridge.begin(); Monitor.begin(115200);
 // For ADC activities, verify analogReadResolution(12) is supported,
 // enable it here and keep ADC_MAX consistent before physical use.
}
void loop() {
 int request=indicator(digitalRead(BUTTON_PIN)==HIGH, digitalRead(ENABLE_PIN)==LOW);
 digitalWrite(LED_BUILTIN,request>0?HIGH:LOW);
 Monitor.println(request);
 // Do not infer buzzer/fan/pump drive approval from this LED surrogate.
}
