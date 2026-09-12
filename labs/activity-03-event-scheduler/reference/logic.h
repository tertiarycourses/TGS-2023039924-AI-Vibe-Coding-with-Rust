#ifndef ACTIVITY_03_LOGIC_H
#define ACTIVITY_03_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* period must be nonzero and below 2^31 ms; next execution follows the actual observed timestamp (no backlog replay). */
int due(uint32_t now, uint32_t *last, uint32_t period);
#ifdef __cplusplus
}
#endif
#endif
