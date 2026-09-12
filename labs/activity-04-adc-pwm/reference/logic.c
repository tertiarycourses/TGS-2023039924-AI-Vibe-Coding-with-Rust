#include "logic.h"
uint8_t pwm_scale(int32_t raw, uint32_t adc_max) {
 if (adc_max == 0 || adc_max > 65535u) return 0;
 if (raw < 0) raw = 0;
 if ((uint32_t)raw > adc_max) raw = (int32_t)adc_max;
 return (uint8_t)(((uint32_t)raw * 255u + adc_max/2u) / adc_max);
}
