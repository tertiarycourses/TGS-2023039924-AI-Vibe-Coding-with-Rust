// Arduino C++ adapter around C-compatible logic. Physical result NOT RUN.
#include "logic.h"
#include <Arduino_RouterBridge.h>
void present_mock(const Sample *sample,uint32_t now) {
 char csv[32];
 if(!sample_valid(sample,now)) { Monitor.println("SAMPLE_REJECT"); return; }
 if(sample_csv(csv,sizeof csv,sample)<0) { Monitor.println("FORMAT_REJECT"); return; }
 Monitor.print("LCD_MOCK:"); Monitor.println(csv);
 Monitor.print("SD_LOG_MOCK:"); Monitor.println(csv);
 // These are console mocks, not a physical LCD or SD write.
}
void setup() {
 Bridge.begin(); Monitor.begin(115200);
 Sample valid={250,60,100}; present_mock(&valid,5100);
 present_mock(&valid,5101); // Deliberately stale: reject.
 Sample wrong_units={2500,60,100}; present_mock(&wrong_units,100);
 Sample wrong_humidity={250,101,100}; present_mock(&wrong_humidity,100);
}
void loop() { /* Deterministic integration fixture runs once. */ }
