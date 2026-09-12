#include "logic.h"
int water_request(int moisture,unsigned age_ms,int sensor_ok) {
 if(sensor_ok!=1 || moisture<0 || moisture>100 || age_ms>2000u) return 0;
 return moisture<30;
}
