#include <stdio.h>

extern int multiply(int x, int y); // Must match Rust signature exactly

int main() {
  int result = multiply(6, 7);
  printf("6 * 7 = %d\n", result);
  return 0;
}

// UB if the function signature mismatches (e.g., wrong types, calling
// convention).
//
// #[no_mangle] is required — omitting it results in linker errors or wrong
// symbols.
//
// Staticlib compilation avoids runtime issues (no dynamic loader problems).
