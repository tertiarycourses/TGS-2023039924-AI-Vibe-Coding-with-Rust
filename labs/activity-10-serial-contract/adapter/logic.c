#include "logic.h"
#include <string.h>
int parse_led(const char *line,int *value) {
 if(strcmp(line,"LED=0")==0) { *value=0; return 1; }
 if(strcmp(line,"LED=1")==0) { *value=1; return 1; }
 return 0;
}
