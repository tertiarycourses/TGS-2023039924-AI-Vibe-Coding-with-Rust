// Arduino C++ adapter around C-compatible logic. Physical result NOT RUN.
#include "logic.h"
#include <Arduino_RouterBridge.h>
char line[16]; unsigned length=0;
bool overflowed=false; int led_value=0;
void setup() {
 Bridge.begin(); Monitor.begin(115200);
 pinMode(LED_BUILTIN,OUTPUT); digitalWrite(LED_BUILTIN,LOW);
}
void loop() {
 unsigned budget=16; // Bounded byte processing per loop pass.
 while(budget-- > 0 && Monitor.available()>0) {
  int incoming=Monitor.read(); if(incoming<0) break;
  char c=(char)incoming;
  if(c=='\n') {
   if(!overflowed) {
    if(length>0 && line[length-1]=='\r') length--;
    line[length]='\0';
    if(parse_led(line,&led_value)) {
     digitalWrite(LED_BUILTIN,led_value?HIGH:LOW);
     Monitor.print("ACCEPT LED="); Monitor.println(led_value);
    } else Monitor.println("REJECT: prior output unchanged");
   } else Monitor.println("REJECT_OVERFLOW: prior output unchanged");
   length=0; overflowed=false;
  } else if(!overflowed) {
   if(length+1u<sizeof line) line[length++]=c;
   else overflowed=true; // Discard whole overlong frame until newline.
  }
 }
}
