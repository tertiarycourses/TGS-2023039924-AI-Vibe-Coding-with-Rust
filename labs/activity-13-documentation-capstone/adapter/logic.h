#ifndef ACTIVITY_13_LOGIC_H
#define ACTIVITY_13_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Named requirement IDs R1/R2/R3 map to dry/wet/fault evidence. safe_request requires enabled and valid moisture; no hardware claims in documentation. */
int safe_request(int enabled, int moisture);
#ifdef __cplusplus
}
#endif
#endif
