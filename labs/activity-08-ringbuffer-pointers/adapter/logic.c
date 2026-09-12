#include "logic.h"
void ring_push(Ring *r, int value) {
 if(value < -1000 || value > 1000) return;
 r->values[r->next] = value; r->next=(r->next+1u)%4u;
 if(r->count<4u) r->count++;
}
int ring_mean(const Ring *r) {
 int sum=0; if(r->count==0) return 0;
 for(unsigned i=0;i<r->count;i++) sum+=r->values[i];
 return sum/(int)r->count;
}
