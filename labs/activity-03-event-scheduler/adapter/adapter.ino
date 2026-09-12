// Arduino C++ wrapper around C-compatible logic; LED surrogate only.
// Verify selected UNO Q core/pinout. Do not connect motors or pumps directly.
#include "logic.h"
#include <Arduino_RouterBridge.h>
const int BUTTON_PIN=2;
static uint32_t started=0,last=0;
static int active=0,previous=0;
void setup() {
 pinMode(LED_BUILTIN,OUTPUT); pinMode(BUTTON_PIN,INPUT_PULLUP);
 Bridge.begin(); Monitor.begin(115200);
 // For ADC activities, verify analogReadResolution(12) is supported,
 // enable it here and keep ADC_MAX consistent before physical use.
}
void loop() {
 int request=due((uint32_t)millis(), &last, 100u);
 digitalWrite(LED_BUILTIN,request>0?HIGH:LOW);
 Monitor.println(request);
 // Do not infer buzzer/fan/pump drive approval from this LED surrogate.
}
