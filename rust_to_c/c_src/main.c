#include <stdio.h>

extern int multiply(int x, int y);

int main() {
  int result = multiply(6, 7);
  printf("6 * 7 = %d\n", result);
  return 0;
}
