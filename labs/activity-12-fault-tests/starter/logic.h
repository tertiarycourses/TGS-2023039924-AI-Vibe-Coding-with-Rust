#ifndef ACTIVITY_12_LOGIC_H
#define ACTIVITY_12_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Moisture 0..100 percent; sensor_ok 0/1. Output is ON only below 30 with age<=2000 ms and sensor_ok true; all other cases OFF. */
int water_request(int moisture, unsigned age_ms, int sensor_ok);
#ifdef __cplusplus
}
#endif
#endif
