#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(water_request(29,2000,1)==1); assert(water_request(30,2000,1)==0);
assert(water_request(29,2001,1)==0); assert(water_request(29,0,0)==0);
assert(water_request(-1,0,1)==0); assert(water_request(101,0,1)==0);
assert(water_request(29,0,2)==0);
puts("PASS Activity 12: Build a fault-injection gate");
return 0;
}
