#ifndef ACTIVITY_02_LOGIC_H
#define ACTIVITY_02_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* raw_high 0 means pressed; enabled 0 always suppresses the output. Invalid levels return 0. */
int indicator(int raw_high, int enabled);
#ifdef __cplusplus
}
#endif
#endif
