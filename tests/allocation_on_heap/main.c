#include <stdlib.h>

int main() {
  int *int_on_heap = malloc(sizeof(int));
  double *double_on_heap = malloc(sizeof(double));

  if (int_on_heap != NULL) {
    *int_on_heap = 256;
  }

  if (double_on_heap != NULL) {
    *double_on_heap = 3.14;
  }

  while (1)
    ;
}
