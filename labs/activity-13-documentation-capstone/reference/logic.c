#include "logic.h"
/* R1: dry soil requests water; R2: wet soil stays off.
 * R3: disabled/invalid sensing fails off. Unit: percent. */
int safe_request(int enabled,int moisture) {
 if(enabled!=1 || moisture<0 || moisture>100) return 0;
 return moisture<30;
}
