#include "logic.h"
#include <stdio.h>
int sample_valid(const Sample *s,uint32_t now) {
 return s->temp_tenths>=-400 && s->temp_tenths<=1250 &&
 s->humidity>=0 && s->humidity<=100 &&
 (uint32_t)(now-s->stamp)<=5000u;
}
int sample_csv(char *out,size_t cap,const Sample *s) {
 int n=snprintf(out,cap,"%d,%d",s->temp_tenths,s->humidity);
 return n<0 || (size_t)n>=cap ? -1 : n;
}
