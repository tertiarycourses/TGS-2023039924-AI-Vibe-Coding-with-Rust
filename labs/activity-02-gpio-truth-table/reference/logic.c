#include "logic.h"
int indicator(int raw_high, int enabled) {
 if ((raw_high != 0 && raw_high != 1) ||
     (enabled != 0 && enabled != 1)) return 0;
 return !raw_high && enabled;
}
