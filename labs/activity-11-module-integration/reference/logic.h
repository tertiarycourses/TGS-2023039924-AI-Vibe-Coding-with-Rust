#ifndef ACTIVITY_11_LOGIC_H
#define ACTIVITY_11_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Sample temperature is tenths of degree, humidity percent 0..100, timestamp ms. Valid temperature -400..1250; stale after 5000 ms. */
typedef struct { int temp_tenths, humidity; uint32_t stamp; } Sample;
int sample_valid(const Sample *s, uint32_t now);
int sample_csv(char *out, size_t cap, const Sample *s);
#ifdef __cplusplus
}
#endif
#endif
