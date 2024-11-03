use std::sync::{Arc, Mutex};

use crate::clock::Subscriber;
use crate::registers;

pub struct Timer {
    rising_edge_notified: std::sync::atomic::AtomicBool,
    registers: Arc<Mutex<registers::Registers>>,
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
    pub fn new(registers: Arc<Mutex<registers::Registers>>) -> Self {
        Self {
            rising_edge_notified: std::sync::atomic::AtomicBool::new(false),
            registers: registers,
            current_clock: 0,
        }
    }

    fn get_prescaler(&self) -> u32 {
        match self.registers.lock().unwrap().io[51] {
            1 => 1,
            2 => 8,
            3 => 64,
            4 => 256,
            5 => 1024,
            _ => 0,
        }
    }
    fn increment_tcnt0(&mut self) -> () {
        if self.registers.lock().unwrap().io[50] == 255 {
            self.registers.lock().unwrap().io[50] = 0;
            let register_value = self.registers.lock().unwrap().io[56];
            self.registers.lock().unwrap().io[56] = register_value | 1;
        } else {
            self.registers.lock().unwrap().io[50] += 1;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_prescaler() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));

        let sut = Timer::new(registers.clone());

        assert_eq!(sut.get_prescaler(), 0);

        registers.lock().unwrap().io[51] = 1;
        assert_eq!(sut.get_prescaler(), 1);

        registers.lock().unwrap().io[51] = 2;
        assert_eq!(sut.get_prescaler(), 8);

        registers.lock().unwrap().io[51] = 3;
        assert_eq!(sut.get_prescaler(), 64);

        registers.lock().unwrap().io[51] = 4;
        assert_eq!(sut.get_prescaler(), 256);

        registers.lock().unwrap().io[51] = 5;
        assert_eq!(sut.get_prescaler(), 1024);
    }

    #[test]
    fn test_run_prescaler_0() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));

        let mut sut = Timer::new(registers.clone());

        for _ in 0..10 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(registers.lock().unwrap().io[50], 0);
        assert_eq!(registers.lock().unwrap().io[56], 0);
    }

    #[test]
    fn test_run_prescaler_1() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));
        registers.lock().unwrap().io[51] = 1;

        let mut sut = Timer::new(registers.clone());

        for _ in 0..255 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(registers.lock().unwrap().io[50], 255);
        assert_eq!(registers.lock().unwrap().io[56], 0);
    }

    #[test]
    fn test_run_prescaler_8() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));
        registers.lock().unwrap().io[51] = 2;

        let mut sut = Timer::new(registers.clone());

        for _ in 0..(255 * 8) {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(registers.lock().unwrap().io[50], 255);
        assert_eq!(registers.lock().unwrap().io[56], 0);
    }

    #[test]
    fn test_run_prescaler_1_overflow() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));
        registers.lock().unwrap().io[51] = 1;

        let mut sut = Timer::new(registers.clone());

        for _ in 0..256 {
            sut.notify_rising_edge();
            sut.run();
        }

        assert_eq!(registers.lock().unwrap().io[50], 0);
        assert_eq!(registers.lock().unwrap().io[56], 1);
    }

    #[test]
    fn test_run_falling_edge() {
        let registers = Arc::new(Mutex::new(registers::Registers::new()));
        registers.lock().unwrap().io[51] = 1;

        let mut sut = Timer::new(registers.clone());

        for _ in 0..10 {
            sut.notify_falling_edge();
            sut.run();
        }

        assert_eq!(registers.lock().unwrap().io[50], 0);
        assert_eq!(registers.lock().unwrap().io[56], 0);
    }
}
