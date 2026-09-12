#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(indicator(0,1)==1); assert(indicator(1,1)==0);
assert(indicator(0,0)==0); assert(indicator(1,0)==0);
assert(indicator(2,1)==0);
puts("PASS Activity 02: Validate a GPIO truth table");
return 0;
}
