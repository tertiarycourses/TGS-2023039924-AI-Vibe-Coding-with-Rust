#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
uint32_t last=0;
assert(due(99,&last,100)==0); assert(due(100,&last,100)==1);
assert(last==100); assert(due(101,&last,100)==0);
assert(due(500,&last,0)==0);
last=UINT32_MAX-50; assert(due(49,&last,100)==1);
puts("PASS Activity 03: Schedule events without delay");
return 0;
}
