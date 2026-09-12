#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(light_percent(0,4095)==0); assert(light_percent(4095,4095)==100);
assert(light_percent(2048,4095)==50); assert(light_percent(-4,4095)==0);
assert(light_percent(10,0)==-1);
puts("PASS Activity 05: Review modular sensor conversion");
return 0;
}
