#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
Ring r={{0},0,0}; assert(ring_mean(&r)==0);
ring_push(&r,10); ring_push(&r,20); assert(ring_mean(&r)==15);
ring_push(&r,30); ring_push(&r,40); ring_push(&r,50);
assert(r.count==4); assert(r.next==1); assert(ring_mean(&r)==35);
ring_push(&r,1001); assert(ring_mean(&r)==35);
puts("PASS Activity 08: Use an array-backed ring buffer");
return 0;
}
