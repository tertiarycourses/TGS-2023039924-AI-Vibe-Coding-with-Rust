#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
char out[16]; assert(status_flags(1,1)==3);
assert(status_flags(0,1)==2); assert(status_flags(0,0)==0);
assert(status_text(out,sizeof out,250)==5);
assert(strcmp(out,"T=250")==0);
assert(status_text(out,3,250)==-1);
puts("PASS Activity 09: Encode bounded status records");
return 0;
}
