use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use bin_file::BinFile;
use clock::Subscriber;
use memory::Memory;

mod clock;
pub mod instruction;
pub mod instruction_executor;
pub mod interrupt_handler;
pub mod memory;
pub mod timer;

pub struct AVREmulator {
    hex_dump: BinFile,
    memory: Arc<Mutex<Memory>>,
    frequency: i64,
    stop_program: Arc<AtomicBool>,
}

impl AVREmulator {
    pub fn new(
        hex_dump: BinFile,
        memory_size: usize,
        frequency: i64,
        stop_program: Arc<AtomicBool>,
    ) -> Self {
        Self {
            hex_dump: hex_dump,
            memory: Arc::new(Mutex::new(Memory::new(memory_size).unwrap())),
            frequency: frequency,
            stop_program: stop_program,
        }
    }

    pub fn run(&self) -> Vec<JoinHandle<()>> {
        let stop_program1 = self.stop_program.clone();
        let stop_program2 = self.stop_program.clone();
        let stop_program3 = self.stop_program.clone();
        let stop_program4 = self.stop_program.clone();

        let instruction_executor: Arc<Mutex<Box<dyn Subscriber>>> = Arc::new(Mutex::new(Box::new(
            instruction_executor::InstructionExecutor::new(
                self.memory.clone(),
                self.hex_dump.clone(),
            ),
        )));

        let timer: Arc<Mutex<Box<dyn Subscriber>>> =
            Arc::new(Mutex::new(Box::new(timer::Timer::new(self.memory.clone()))));

        let interrupt_handler: Arc<Mutex<Box<dyn Subscriber>>> = Arc::new(Mutex::new(Box::new(
            interrupt_handler::InterruptHandler::new(self.memory.clone()),
        )));

        let clock = Arc::new(Mutex::new(clock::Clock::new(self.frequency as f64)));

        clock
            .lock()
            .unwrap()
            .subscribe(instruction_executor.clone());
        clock.lock().unwrap().subscribe(timer.clone());
        clock.lock().unwrap().subscribe(interrupt_handler.clone());

        let instruction_executor_thread = std::thread::spawn(move || loop {
            instruction_executor.lock().unwrap().run();
            if stop_program1.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        });

        let clock_thread = std::thread::spawn(move || loop {
            clock.lock().unwrap().run();
            if stop_program2.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        });

        let timer_thread = std::thread::spawn(move || loop {
            timer.lock().unwrap().run();
            if stop_program3.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        });

        let interrupt_handler_thread = std::thread::spawn(move || loop {
            interrupt_handler.lock().unwrap().run();
            if stop_program4.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        });

        vec![
            instruction_executor_thread,
            clock_thread,
            timer_thread,
            interrupt_handler_thread,
        ]
    }
}
