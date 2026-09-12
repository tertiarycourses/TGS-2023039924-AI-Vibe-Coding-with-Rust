#include "logic.h"
#include "logic.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void) {
assert(pwm_scale(0,4095)==0); assert(pwm_scale(4095,4095)==255);
assert(pwm_scale(2048,4095)==128); assert(pwm_scale(-1,4095)==0);
assert(pwm_scale(5000,4095)==255); assert(pwm_scale(1,0)==0);
assert(pwm_scale(65535,65535)==255);
puts("PASS Activity 04: Scale an ADC reading to PWM");
return 0;
}
