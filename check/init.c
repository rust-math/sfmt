
#define SFMT_MEXP 19937
#include "SFMT-src-1.5.1/SFMT.h"

void print128(__m128i var) {
  uint32_t *val = (uint32_t *)&var;
  printf("%i %i %i %i\n", val[0], val[1], val[2], val[3]);
}

int main() {
  sfmt_t sfmt;
  sfmt_init_gen_rand(&sfmt, 1234);
  for (int i = 0; i < SFMT_N; i++) {
    print128(sfmt.state[i].si);
  }
}
