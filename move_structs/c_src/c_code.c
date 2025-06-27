#include <stdio.h>

typedef struct {
  int x;
  int y;
} Point;

void print_point(Point pt) { printf("Point: (%d, %d)\n", pt.x, pt.y); }
