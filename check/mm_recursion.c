
#define SFMT_MEXP 19937
#include "SFMT-src-1.5.1/SFMT.h"
#include <stdio.h>

static const w128_t sse2_param_mask = {
    {SFMT_MSK1, SFMT_MSK2, SFMT_MSK3, SFMT_MSK4}};

#include "SFMT-src-1.5.1/SFMT-sse2.h"

void print128(__m128i var) {
  uint32_t *val = (uint32_t *)&var;
  printf("%i %i %i %i\n", val[0], val[1], val[2], val[3]);
}

int main() {
  const __m128i a = _mm_setr_epi32(1, 2, 3, 4);
  const __m128i b = _mm_setr_epi32(431, 232, 83, 14);
  const __m128i c = _mm_setr_epi32(213, 22, 93, 234);
  const __m128i d = _mm_setr_epi32(112, 882, 23, 124);
  __m128i r;
  mm_recursion(&r, a, a, a, a);
  print128(r);
  mm_recursion(&r, a, b, c, d);
  print128(r);
}
