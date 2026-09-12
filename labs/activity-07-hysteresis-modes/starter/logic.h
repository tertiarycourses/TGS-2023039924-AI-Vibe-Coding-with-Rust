#ifndef ACTIVITY_07_LOGIC_H
#define ACTIVITY_07_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Temperature uses tenths of degree; AUTO on at >=300, off at <=280, retained state between. Unknown mode fails OFF. */
int fan_next(int mode, int temp_tenths, int previous);
#ifdef __cplusplus
}
#endif
#endif
