#ifndef ACTIVITY_10_LOGIC_H
#define ACTIVITY_10_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Input is NUL-terminated exact text, line terminator removed by transport. Parser does not accept spaces, extra tokens or unbounded commands. */
int parse_led(const char *line, int *value);
#ifdef __cplusplus
}
#endif
#endif
