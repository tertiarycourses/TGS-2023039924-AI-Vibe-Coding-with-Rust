#include "logic.h"
int doorbell(uint32_t now, int pressed, uint32_t *started, int *active) {
 if (pressed) { *started = now; *active = 1; }
 if (*active && (uint32_t)(now - *started) >= 2000u) *active = 0;
 return *active;
}
