#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(proposal_led(1,80,1000)==1); assert(proposal_led(0,100,0)==0);
assert(proposal_led(1,79,0)==-1); assert(proposal_led(2,99,0)==-1);
assert(proposal_led(1,101,0)==-1); assert(proposal_led(1,90,1001)==-1);
puts("PASS Activity 14: Bound a UNO Q local-AI suggestion");
return 0;
}
