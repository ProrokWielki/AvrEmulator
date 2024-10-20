int main() {
  int b = 2;
  if (--b) {
    asm("nop");
  }
  if (!b--) {
    asm("nop");
  }
  if (!b--) {
    asm("nop");
  }

  while(1);
}