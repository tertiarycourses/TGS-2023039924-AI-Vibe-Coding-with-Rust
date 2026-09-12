#ifndef ACTIVITY_14_LOGIC_H
#define ACTIVITY_14_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Proposal value is 0/1; confidence integer 0..100, minimum 80; age<=1000 ms. Invalid suggestion returns -1 and must not reach an actuator. */
int proposal_led(int value, int confidence, unsigned age_ms);
#ifdef __cplusplus
}
#endif
#endif
