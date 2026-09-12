#ifndef ACTIVITY_06_LOGIC_H
#define ACTIVITY_06_LOGIC_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* alarm() accepts signed whole-degree temperature and limit; equality is alarm-on. No hidden global threshold. */
int alarm(int temperature, int limit);
#ifdef __cplusplus
}
#endif
#endif
