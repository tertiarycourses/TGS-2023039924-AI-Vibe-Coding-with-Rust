#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
int value=7; assert(parse_led("LED=1",&value)==1 && value==1);
assert(parse_led("LED=0",&value)==1 && value==0);
value=7; assert(parse_led("LED=2",&value)==0 && value==7);
assert(parse_led("LED=1;PUMP=1",&value)==0);
assert(parse_led(" LED=1",&value)==0);
puts("PASS Activity 10: Parse a bounded serial command");
return 0;
}
