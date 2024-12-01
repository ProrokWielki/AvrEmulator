use std::sync::{Arc, Mutex};

use crate::avr_emulator::clock::Subscriber;
use crate::avr_emulator::memory::Memory;

pub struct Timer {
    rising_edge_notified: std::sync::atomic::AtomicBool,
    memory: Arc<Mutex<Memory>>,
    current_clock: u32,
}

impl<'a> Subscriber for Timer {
    fn notify_rising_edge(&self) {
        log::debug!("InstructionExecutor rising edge notified");

        if self
            .rising_edge_notified
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            log::warn!("Timer did not finish handling previous rising edge!");
        }

        self.rising_edge_notified
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    fn notify_falling_edge(&self) {}

    fn run(&mut self) {
        if self
            .rising_edge_notified
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let prescaler = self.get_prescaler();

            if prescaler != 0 {
                self.current_clock += 1;

                if self.current_clock % prescaler == 0 || self.current_clock > prescaler {
                    self.current_clock = 0;
                    self.increment_tcnt0();
                }
            }

            self.rising_edge_notified
                .store(false, std::sync::atomic::Ordering::Relaxed)
        }
    }
}

impl Timer {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        Self {
            rising_edge_notified: std::sync::atomic::AtomicBool::new(false),
            memory: memory,
            current_clock: 0,
        }
    }

    fn get_prescaler(&self) -> u32 {
        match self.memory.lock().unwrap().get_io(51).unwrap() {
            1 => 1,
            2 => 8,
            3 => 64,
            4 => 256,
            5 => 1024,
            _ => 0,
        }
    }
    fn increment_tcnt0(&mut self) -> () {
        if self.memory.lock().unwrap().get_io(50).unwrap() == 255 {
            self.memory.lock().unwrap().set_io(50, 0);
            let register_value = self.memory.lock().unwrap().get_io(56).unwrap();
            self.memory.lock().unwrap().set_io(56, register_value | 1);
        } else {
            let register_value = self.memory.lock().unwrap().get_io(50).unwrap();
            self.memory.lock().unwrap().set_io(50, register_value + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prescaler() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));

        let sut = Timer::new(memory.clone());

        assert_eq!(sut.get_prescaler(), 0);

        memory.lock().unwrap().set_io(51, 1);
        assert_eq!(sut.get_prescaler(), 1);

        memory.lock().unwrap().set_io(51, 2);
        assert_eq!(sut.get_prescaler(), 8);

        memory.lock().unwrap().set_io(51, 3);
        assert_eq!(sut.get_prescaler(), 64);

        memory.lock().unwrap().set_io(51, 4);
        assert_eq!(sut.get_prescaler(), 256);

        memory.lock().unwrap().set_io(51, 5);
        assert_eq!(sut.get_prescaler(), 1024);
    }

    #[test]
    fn test_run_prescaler_0() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));

        let mut sut = Timer::new(memory.clone());

        for _ in 0..10 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(memory.lock().unwrap().get_io(50).unwrap(), 0);
        assert_eq!(memory.lock().unwrap().get_io(56).unwrap(), 0);
    }

    #[test]
    fn test_run_prescaler_1() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));
        memory.lock().unwrap().set_io(51, 1);

        let mut sut = Timer::new(memory.clone());

        for _ in 0..255 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(memory.lock().unwrap().get_io(50).unwrap(), 255);
        assert_eq!(memory.lock().unwrap().get_io(56).unwrap(), 0);
    }

    #[test]
    fn test_run_prescaler_8() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));
        memory.lock().unwrap().set_io(51, 2);

        let mut sut = Timer::new(memory.clone());

        for _ in 0..(255 * 8) {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(memory.lock().unwrap().get_io(50).unwrap(), 255);
        assert_eq!(memory.lock().unwrap().get_io(56).unwrap(), 0);
    }

    #[test]
    fn test_run_prescaler_1_overflow() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));
        memory.lock().unwrap().set_io(51, 1);

        let mut sut = Timer::new(memory.clone());

        for _ in 0..256 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(memory.lock().unwrap().get_io(50).unwrap(), 0);
        assert_eq!(memory.lock().unwrap().get_io(56).unwrap(), 1);
    }

    #[test]
    fn test_run_falling_edge() {
        let memory = Arc::new(Mutex::new(Memory::new(100, vec![]).unwrap()));
        memory.lock().unwrap().set_io(51, 1);

        let mut sut = Timer::new(memory.clone());

        for _ in 0..10 {
            sut.notify_falling_edge();
            sut.run();
        }

        assert_eq!(memory.lock().unwrap().get_io(50).unwrap(), 0);
        assert_eq!(memory.lock().unwrap().get_io(56).unwrap(), 0);
    }
}
