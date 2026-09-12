#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
Sample s={250,60,100}; char out[32];
assert(sample_valid(&s,5100)==1); assert(sample_valid(&s,5101)==0);
assert(sample_csv(out,sizeof out,&s)==6); assert(strcmp(out,"250,60")==0);
s.humidity=101; assert(sample_valid(&s,100)==0);
s.humidity=60; s.temp_tenths=2500; assert(sample_valid(&s,100)==0);
assert(sample_csv(out,2,&s)==-1);
puts("PASS Activity 11: Integrate sensor display and logging");
return 0;
}
