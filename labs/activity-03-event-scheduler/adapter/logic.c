#include "logic.h"
int due(uint32_t now, uint32_t *last, uint32_t period) {
 if (period == 0u || period >= 0x80000000u) return 0;
 if ((uint32_t)(now - *last) < period) return 0;
 *last = now; return 1;
}
