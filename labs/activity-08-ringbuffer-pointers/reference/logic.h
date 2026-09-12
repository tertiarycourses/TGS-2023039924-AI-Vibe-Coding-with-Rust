#ifndef ACTIVITY_08_LOGIC_H
#define ACTIVITY_08_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Buffer capacity is 4; count 0..4; next 0..3; oldest sample overwritten. Empty mean returns 0. Samples are restricted to -1000..1000. */
typedef struct { int values[4]; unsigned next, count; } Ring;
void ring_push(Ring *r, int value);
int ring_mean(const Ring *r);
#ifdef __cplusplus
}
#endif
#endif
