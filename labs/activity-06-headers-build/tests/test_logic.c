#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(alarm(29,30)==0); assert(alarm(30,30)==1);
assert(alarm(-10,0)==0); assert(alarm(80,30)==1);
puts("PASS Activity 06: Repair a header and build contract");
return 0;
}
