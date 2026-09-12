#include "logic.h"
int fan_next(int mode, int temp_tenths, int previous) {
 switch (mode) {
 case 0: if (temp_tenths>=300) return 1;
         if (temp_tenths<=280) return 0;
         return previous != 0;
 case 1: return 1;
 case 2: return 0;
 default: return 0;
 }
}
