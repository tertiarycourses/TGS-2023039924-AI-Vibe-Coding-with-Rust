#ifndef ACTIVITY_05_LOGIC_H
#define ACTIVITY_05_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Maximum 1..65535; clamped raw input; rounded integer percentage 0..100. No Serial, GPIO or allocation in this module. */
int light_percent(int raw, unsigned maximum);
#ifdef __cplusplus
}
#endif
#endif
