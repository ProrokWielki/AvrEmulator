use std::sync::{Arc, Mutex};

use crate::clock;
use crate::instruction;
use crate::memory::Memory;

pub struct InstructionExecutor {
    rising_edge_notified: std::sync::atomic::AtomicBool,
    memory: Arc<Mutex<Memory>>,
    hex_dump: bin_file::BinFile,
}

impl InstructionExecutor {
    pub fn new(memory: Arc<Mutex<Memory>>, hex_dump: bin_file::BinFile) -> Self {
        Self {
            rising_edge_notified: std::sync::atomic::AtomicBool::new(false),
            memory: memory,
            hex_dump: hex_dump,
        }
    }

    fn get_current_instruction_opcode(&self) -> u16 {
        let offset = (self.memory.lock().unwrap().pc * 2) as usize;

        let a = self.hex_dump.get_value_by_address(offset).unwrap() as u16;
        let b = self.hex_dump.get_value_by_address(offset + 1).unwrap() as u16;

        (b << 8 | a) as u16
    }

    fn find_instruction_from_opcode(&mut self, opcode: u16) -> Box<dyn instruction::Instruction> {
        match instruction::get_instruction(opcode) {
            None => {
                log::error!("unknown opcode: {:#06x}", opcode);
                std::process::exit(2); // TODO: this should be handled elsewhere ...
            }
            Some(instruction) => {
                log::info!("instruction: {}", instruction.str());
                instruction
            }
        }
    }
}

impl<'a> clock::Subscriber for InstructionExecutor {
    fn notify_rising_edge(&self) {
        log::debug!("InstructionExecutor rising edge notified");

        if self
            .rising_edge_notified
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            log::warn!("InstructionExecutor did not finish handling previous rising edge!");
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
            let current_instruction_opcode = self.get_current_instruction_opcode();
            let current_instruction = self.find_instruction_from_opcode(current_instruction_opcode);
            current_instruction.process(&mut self.memory.lock().unwrap());

            self.rising_edge_notified
                .store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::Subscriber;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_run_without_notify() {
        let empty_registers = Arc::new(Mutex::new(Memory::new(100).unwrap()));

        let mut sut = InstructionExecutor::new(
            Arc::new(Mutex::new(Memory::new(100).unwrap())),
            bin_file::BinFile::new(),
        );

        sut.run();

        assert_eq!(
            *empty_registers.lock().unwrap(),
            *sut.memory.lock().unwrap()
        );
    }

    #[test]
    fn test_run_with_falling_edge_notify() {
        let empty_registers = Arc::new(Mutex::new(Memory::new(100).unwrap()));

        let mut sut = InstructionExecutor::new(
            Arc::new(Mutex::new(Memory::new(100).unwrap())),
            bin_file::BinFile::new(),
        );

        sut.notify_falling_edge();
        sut.run();

        assert_eq!(
            *empty_registers.lock().unwrap(),
            *sut.memory.lock().unwrap()
        );
    }

    #[test]
    fn test_run_with_rising_edge_notify() {
        let expected_registers = Arc::new(Mutex::new(Memory::new(100).unwrap()));
        expected_registers.lock().unwrap().pc = 1;

        let mut stub_bin_file = bin_file::BinFile::new();
        let _ = stub_bin_file.add_bytes([0, 0], Some(0), true);

        let mut sut = InstructionExecutor::new(
            Arc::new(Mutex::new(Memory::new(100).unwrap())),
            stub_bin_file,
        );

        sut.notify_rising_edge();
        sut.run();

        assert_eq!(
            *expected_registers.lock().unwrap(),
            *sut.memory.lock().unwrap()
        );
    }

    #[test]
    fn find_instruction_from_opcode() {
        let expected_registers = Arc::new(Mutex::new(Memory::new(100).unwrap()));
        expected_registers.lock().unwrap().pc = 1;

        let mut stub_bin_file = bin_file::BinFile::new();
        let _ = stub_bin_file.add_bytes([0, 0], Some(0), true);

        let mut sut = InstructionExecutor::new(
            Arc::new(Mutex::new(Memory::new(100).unwrap())),
            stub_bin_file,
        );

        sut.notify_rising_edge();
        sut.run();

        assert_eq!(
            *expected_registers.lock().unwrap(),
            *sut.memory.lock().unwrap()
        );
    }
}
