use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::sync::{Arc, Mutex};

use crate::avr_emulator::clock;
use crate::avr_emulator::memory::{Memory, SregBit};

pub struct InterruptHandler {
    rising_edge_notified: std::sync::atomic::AtomicBool,
    memory: Arc<Mutex<Memory>>,
}

// TODO: this is for atmega8. Is it valid for all AVRs? - probably not - Make it generic
#[derive(EnumIter)]
enum Interrupt {
    Reset,
    Int0,
    Int1,
    Timer2Comp,
    Timer2Ovf,
    Timer1Capt,
    Timer1CompA,
    Timer1ComB,
    Timer1Ovf,
    Timer0Ovf,
    SPISTC,
    USARTRXC,
    USARTUDRE,
    USARTTXC,
    ADC,
    EERDY,
    ANACOMP,
    TWI,
    SPMRdy,
}

impl clock::Subscriber for InterruptHandler {
    fn notify_rising_edge(&self) {
        log::debug!("InterruptHandler rising edge notified");

        if self
            .rising_edge_notified
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            log::warn!("InterruptHandler did not finish handling previous rising edge!");
        }

        self.rising_edge_notified
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    fn notify_falling_edge(&self) {}

    fn run(&mut self) {
        // TODO: make sure it blocks instruction execution when the pc is changed
        // TODO: make sure a single instruction is performed between execution of 2 interrupts
        if self
            .rising_edge_notified
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            if self.are_interrupts_enabled() {
                let current_interrupt_maybe = self.get_current_interrupt();

                if current_interrupt_maybe.is_some() {
                    self.disable_interrupts();

                    let current_interrupt = current_interrupt_maybe.unwrap();

                    let interrupt_routine_address =
                        self.get_interrupt_routine_address(&current_interrupt);

                    self.clear_interrupt_flag(&current_interrupt); // TODO: is it in the right place?

                    self.execute_interrupt_routine(interrupt_routine_address);
                }
            }
            self.rising_edge_notified
                .store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

impl InterruptHandler {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        Self {
            rising_edge_notified: std::sync::atomic::AtomicBool::new(false),
            memory: memory,
        }
    }

    fn disable_interrupts(&mut self) {
        self.memory
            .lock()
            .unwrap()
            .clear_status_register_bit(SregBit::I);
    }

    fn clear_interrupt_flag(&mut self, interrupt: &Interrupt) {
        match interrupt {
            Interrupt::Timer0Ovf => {
                let register_value = self.memory.lock().unwrap().get_io(56).unwrap();
                self.memory
                    .lock()
                    .unwrap()
                    .set_io(56, register_value & (!1u8));
            }
            _ => (),
        }
    }

    fn are_interrupts_enabled(&self) -> bool {
        self.memory
            .lock()
            .unwrap()
            .get_status_register_bit(SregBit::I)
    }

    fn get_current_interrupt(&self) -> Option<Interrupt> {
        for interrupt in Interrupt::iter() {
            if self.is_enabled(&interrupt) && self.occurred(&interrupt) {
                return Some(interrupt);
            }
        }
        None
    }

    fn get_interrupt_routine_address(&self, interrupt: &Interrupt) -> u16 {
        match interrupt {
            Interrupt::Timer0Ovf => 0x0009,
            _ => 0,
        }
    }

    fn is_enabled(&self, interrupt: &Interrupt) -> bool {
        match interrupt {
            Interrupt::Timer0Ovf => (self.memory.lock().unwrap().get_io(57).unwrap() & 1) == 1,
            _ => false,
        }
    }

    fn occurred(&self, interrupt: &Interrupt) -> bool {
        match interrupt {
            Interrupt::Timer0Ovf => (self.memory.lock().unwrap().get_io(56).unwrap() & 1) == 1,
            _ => false,
        }
    }

    fn execute_interrupt_routine(&mut self, routine_address: u16) {
        log::error!("executing {} interrupt", routine_address);

        let sp = self.memory.lock().unwrap().get_sp();
        let pc = self.memory.lock().unwrap().get_pc();

        self.memory
            .lock()
            .unwrap()
            .set_stack(sp as usize, (pc & (0xff00 >> 8)) as u8);

        self.memory
            .lock()
            .unwrap()
            .set_stack(sp as usize, (pc & (0x00ff)) as u8);

        self.memory.lock().unwrap().set_sp(sp - 2);

        self.memory.lock().unwrap().set_pc(routine_address);
    }
}

#[cfg(test)]
mod tests {
    use clock::Subscriber;

    use super::*;

    use std::sync::{Arc, Mutex};

    #[test]
    fn test_interrupts_are_disabled_after_interrupt_occurs() {
        let memory = Arc::new(Mutex::new(Memory::new(200, vec![]).unwrap()));
        memory.lock().unwrap().set_status_register_bit(SregBit::I);
        memory.lock().unwrap().set_io(57, 1);
        memory.lock().unwrap().set_io(56, 1);
        memory.lock().unwrap().set_sp(50);
        memory.lock().unwrap().set_pc(30);

        let mut sut = InterruptHandler::new(memory.clone());
        sut.notify_rising_edge();
        sut.run();

        assert!(!memory.lock().unwrap().get_status_register_bit(SregBit::I));
        assert!(!memory.lock().unwrap().get_sp() != 50);
    }

    #[test]
    fn test_interrupts_stay_enabled_if_interrupt_does_not_occur() {
        let memory = Arc::new(Mutex::new(Memory::new(200, vec![]).unwrap()));
        memory.lock().unwrap().set_status_register_bit(SregBit::I);

        let mut sut = InterruptHandler::new(memory.clone());
        sut.notify_rising_edge();
        sut.run();

        assert!(memory.lock().unwrap().get_status_register_bit(SregBit::I));
    }

    #[test]
    fn test_interrupt_routine_is_not_executed_if_interrupts_are_disabled() {
        let memory = Arc::new(Mutex::new(Memory::new(200, vec![]).unwrap()));
        memory.lock().unwrap().clear_status_register_bit(SregBit::I);
        memory.lock().unwrap().set_io(57, 1);
        memory.lock().unwrap().set_io(56, 1);
        memory.lock().unwrap().set_sp(50);
        memory.lock().unwrap().set_pc(40);

        let mut sut = InterruptHandler::new(memory.clone());
        sut.notify_rising_edge();
        sut.run();

        assert_eq!(memory.lock().unwrap().get_sp(), 50);
    }
}
