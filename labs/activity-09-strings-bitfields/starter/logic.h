#ifndef ACTIVITY_09_LOGIC_H
#define ACTIVITY_09_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Temperature text fits caller capacity; snprintf truncation returns -1. Status bit 0=fan, bit 1=fault; other bits zero. */
uint8_t status_flags(int fan, int fault);
int status_text(char *out, size_t cap, int temp);
#ifdef __cplusplus
}
#endif
#endif
