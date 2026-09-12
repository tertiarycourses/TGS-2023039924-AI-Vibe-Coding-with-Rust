#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(safe_request(1,29)==1); assert(safe_request(1,30)==0);
assert(safe_request(0,29)==0); assert(safe_request(1,-1)==0);
assert(safe_request(1,101)==0);
puts("PASS Activity 13: Document an irrigation interface");
return 0;
}
