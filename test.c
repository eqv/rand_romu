#include <stdint.h>
#include <stdio.h>

#define ROTL(d,lrot)  ((d<<(lrot)) | (d>>(8*sizeof(d)-(lrot))))

uint64_t xState, yState;  // set to nonzero seed

uint64_t romuDuoJr_random () {
   uint64_t xp = xState;
   xState = 15241094284759029579u * yState;
   yState = yState - xp;  yState = ROTL(yState,27);
   return xp;
}

void main(int argc, char** argv){
  xState = 0x3c91b13ee3913664UL;
  yState = 0x863f0e37c2637d1fUL;

  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
  printf("%lx\n", romuDuoJr_random());
}
