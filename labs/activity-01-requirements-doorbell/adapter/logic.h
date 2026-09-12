#ifndef ACTIVITY_01_LOGIC_H
#define ACTIVITY_01_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Input pressed is 0/1; timestamp is uint32_t milliseconds. A new press restarts the 2000 ms interval; output is 0 at the deadline. */
int doorbell(uint32_t now, int pressed, uint32_t *started, int *active);
#ifdef __cplusplus
}
#endif
#endif
