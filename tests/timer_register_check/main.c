#include <avr/io.h>

void main() {
  TCCR0 = (1 << CS01) | (1 << CS00);

  while (!(TIFR & (1 << TOV0)) && TCNT0 < 250) {
  }

  while (1) {
  }
}
