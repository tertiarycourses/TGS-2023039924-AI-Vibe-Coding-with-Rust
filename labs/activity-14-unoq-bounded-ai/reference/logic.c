#include "logic.h"
int proposal_led(int value,int confidence,unsigned age_ms) {
 if((value!=0 && value!=1) || confidence<80 || confidence>100 || age_ms>1000u) return -1;
 return value;
}
