// Arduino C++ wrapper around C-compatible logic; LED surrogate only.
// Verify selected UNO Q core/pinout. Do not connect motors or pumps directly.
#include "logic.h"
#include <Arduino_RouterBridge.h>
const int BUTTON_PIN=2;
const uint32_t ADC_MAX=4095u;
void setup() {
 pinMode(LED_BUILTIN,OUTPUT); pinMode(BUTTON_PIN,INPUT_PULLUP);
 Bridge.begin(); Monitor.begin(115200);
 analogReadResolution(12); // Selected UNO Q core must support this setting.
}
void loop() {
 int raw=analogRead(A0);
 if(raw<0) { digitalWrite(LED_BUILTIN,LOW); Monitor.println("ADC_ERROR"); return; }
 int request=light_percent(raw, ADC_MAX);
 digitalWrite(LED_BUILTIN,request>0?HIGH:LOW);
 Monitor.print("raw="); Monitor.print(raw);
 Monitor.print(", adc_max="); Monitor.print(ADC_MAX);
 Monitor.print(", scaled="); Monitor.println(request);
 // Do not infer buzzer/fan/pump drive approval from this LED surrogate.
}
