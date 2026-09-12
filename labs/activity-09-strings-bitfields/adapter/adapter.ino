// Arduino C++ adapter around C-compatible logic. Physical result NOT RUN.
#include "logic.h"
#include <Arduino_RouterBridge.h>
void setup() {
 Bridge.begin(); Monitor.begin(115200);
 char text[16]; int n=status_text(text,sizeof text,250);
 if(n<0) Monitor.println("FORMAT_REJECT");
 else { Monitor.print("mock status="); Monitor.println(text); }
 Monitor.print("flags="); Monitor.println((unsigned)status_flags(1,1));
 // Expected T=250 and flags=3; no raw struct/bitfield serialisation.
}
void loop() { /* Fixture printed once; no LCD rendering claim. */ }
