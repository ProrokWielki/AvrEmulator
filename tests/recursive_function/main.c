
int fibonacci(unsigned int n) {
  if (n < 0) {
    return 0;
  }

  if (n == 0 || n == 1) {
    return 1;
  }

  return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
  int result = fibonacci(9);
  while (1)
    ;
}
