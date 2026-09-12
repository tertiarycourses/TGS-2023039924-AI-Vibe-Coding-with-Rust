#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(fan_next(0,300,0)==1); assert(fan_next(0,290,1)==1);
assert(fan_next(0,280,1)==0); assert(fan_next(0,290,0)==0);
assert(fan_next(1,0,0)==1); assert(fan_next(2,500,1)==0);
assert(fan_next(9,500,1)==0);
puts("PASS Activity 07: Implement fan modes and hysteresis");
return 0;
}
