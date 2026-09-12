#include "logic.h"
#include <stdio.h>
uint8_t status_flags(int fan,int fault) {
 return (uint8_t)((fan?1u:0u) | (fault?2u:0u));
}
int status_text(char *out,size_t cap,int temp) {
 int n=snprintf(out,cap,"T=%d",temp);
 return n<0 || (size_t)n>=cap ? -1 : n;
}
