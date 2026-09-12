// Arduino C++ adapter around C-compatible logic. Physical result NOT RUN.
#include "logic.h"
#include <Arduino_RouterBridge.h>
uint32_t received_ms=0;
bool leased=false; int permitted=0,led_state=0;
int review_led(int value,int confidence) {
 int request=proposal_led(value,confidence,0u);
 if(request<0) {
  leased=false; permitted=0; led_state=0;
  digitalWrite(LED_BUILTIN,LOW); return 0;
 }
 permitted=request; received_ms=(uint32_t)millis(); leased=true;
 return 1; // Accepted; Python must read back actual state separately.
}
int read_led() { return led_state; }
void setup() {
 Bridge.begin(); Monitor.begin(115200);
 pinMode(LED_BUILTIN,OUTPUT); digitalWrite(LED_BUILTIN,LOW);
 Bridge.provide_safe("review_led",review_led);
 Bridge.provide_safe("read_led",read_led);
 const int proposals[6][3]={{1,80,1000},{0,100,0},{1,79,0},
                           {2,99,0},{1,101,0},{1,90,1001}};
 for(unsigned i=0;i<6;i++) {
  int result=proposal_led(proposals[i][0],proposals[i][1],(unsigned)proposals[i][2]);
  Monitor.print("mock proposal result="); Monitor.println(result);
 }
 Monitor.println("Expected1,0,-1,-1,-1,-1. No AI inference or RPC round trip claimed.");
}
void loop() {
 uint32_t age=(uint32_t)((uint32_t)millis()-received_ms);
 led_state=leased && age<=1000u ? permitted : 0;
 if(age>1000u) leased=false;
 digitalWrite(LED_BUILTIN,led_state?HIGH:LOW);
 // Only bounded LED leases; no pump, shell, network or model text commands.
}
