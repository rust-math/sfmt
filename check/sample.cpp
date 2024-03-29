/**
 * Generate u64 random integers using original SFMT implementation
 *
 * - Seed is fixed value (seed=1234)
 * - Generate 10000 integers
 * - `SFMT_MEXP` will be set as a compiler flag. See Makefile.
 */
#include "./SFMT-src-1.5.1/SFMT.h"
#include <iostream>

int main(int argc, char *argv[]) {
  sfmt_t sfmt;
  sfmt_init_gen_rand(&sfmt, 1234);
  for (int i = 0; i < 10000; i++) {
    uint64_t x = sfmt_genrand_uint64(&sfmt);
    std::cout << x << "\n";
  }
  std::cout << std::flush;
  return 0;
}
