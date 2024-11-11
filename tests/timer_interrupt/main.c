#include <avr/interrupt.h>
#include <avr/io.h>

void main() {
  TIMSK |= (1 << TOIE0);
  TCCR0 |= (1 << CS01) | (1 << CS00);
  sei();

  while (1) {
  }
}

ISR(TIMER0_OVF_vect) { asm("nop"); }