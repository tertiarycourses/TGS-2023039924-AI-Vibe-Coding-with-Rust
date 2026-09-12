#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
uint32_t start=0; int active=0;
assert(doorbell(10,1,&start,&active)==1);
assert(doorbell(2009,0,&start,&active)==1);
assert(doorbell(2010,0,&start,&active)==0);
assert(doorbell(UINT32_MAX-100,1,&start,&active)==1);
assert(doorbell(1899,0,&start,&active)==0);
puts("PASS Activity 01: Specify and model a doorbell");
return 0;
}
