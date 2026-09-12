#include "logic.h"
int light_percent(int raw, unsigned maximum) {
 if (maximum==0 || maximum>65535u) return -1;
 if (raw<0) raw=0;
 if ((unsigned)raw>maximum) raw=(int)maximum;
 return (int)(((unsigned)raw*100u+maximum/2u)/maximum);
}
