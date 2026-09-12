#ifndef ACTIVITY_04_LOGIC_H
#define ACTIVITY_04_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* raw is clamped to 0..adc_max; adc_max 1..65535; result rounded to nearest 0..255. Return 0 for invalid adc_max. */
uint8_t pwm_scale(int32_t raw, uint32_t adc_max);
#ifdef __cplusplus
}
#endif
#endif
