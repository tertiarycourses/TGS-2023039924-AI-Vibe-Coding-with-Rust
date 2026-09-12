// Arduino C++ adapter around C-compatible logic. Physical result NOT RUN.
#include "logic.h"
#include <Arduino_RouterBridge.h>
Ring samples={{0},0,0};
void setup() {
 Bridge.begin(); Monitor.begin(115200);
 const int values[5]={10,20,30,40,50};
 for(unsigned i=0;i<5;i++) {
  ring_push(&samples,values[i]);
  Monitor.print("mock sample="); Monitor.print(values[i]);
  Monitor.print(", mean="); Monitor.println(ring_mean(&samples));
 }
 Monitor.println("Expected final mean=35; deterministic fixture, not sensor measurement.");
}
void loop() { /* No waiting, allocation or repeated insertion. */ }
