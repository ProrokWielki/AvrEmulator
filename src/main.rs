use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use structopt::StructOpt;

use clock::Subscriber;
use memory::Memory;

pub mod clock;
pub mod instruction;
pub mod instruction_executor;
pub mod memory;
pub mod timer;

#[derive(Debug, StructOpt)]
#[structopt(name = "AVRSimulator", about = "allows running avr hex")]
struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    /// Verbose mode (-v, -vv, -vvv, etc.)
    verbose: u8,

    #[structopt(short, long, default_value = "1000000")]
    /// clock frequency in Hz
    frequency: i64,

    /// hex file to be "executed"
    #[structopt(name = "FILE", parse(from_os_str))]
    file_name: PathBuf,
}

fn to_filter_level(verbose: u8) -> log::LevelFilter {
    let levels = [
        log::LevelFilter::Error,
        log::LevelFilter::Warn,
        log::LevelFilter::Info,
        log::LevelFilter::Debug,
        log::LevelFilter::Trace,
    ];
    levels[verbose as usize]
}

fn main() {
    let opt = Opt::from_args();

    env_logger::Builder::from_default_env()
        .filter_level(to_filter_level(opt.verbose))
        .init();

    let mut file_path = std::env::current_dir().unwrap();

    if opt.file_name.is_relative() {
        file_path.push(opt.file_name);
    } else {
        file_path = opt.file_name;
    }

    let hex_dump = bin_file::BinFile::from_file(Path::new(&file_path)).unwrap();
    let memory = Arc::new(Mutex::new(Memory::new(2000).unwrap()));

    let instruction_executor: Arc<Mutex<Box<dyn Subscriber>>> = Arc::new(Mutex::new(Box::new(
        instruction_executor::InstructionExecutor::new(memory.clone(), hex_dump),
    )));

    let timer: Arc<Mutex<Box<dyn Subscriber>>> =
        Arc::new(Mutex::new(Box::new(timer::Timer::new(memory.clone()))));

    let mut clock = clock::Clock::new(opt.frequency as f64);

    clock.subscribe(instruction_executor.clone());
    clock.subscribe(timer.clone());

    let instruction_executor_thread = std::thread::spawn(move || loop {
        instruction_executor.lock().unwrap().run();
    });
    let clock_thread = std::thread::spawn(move || loop {
        clock.run();
    });
    let timer_thread = std::thread::spawn(move || loop {
        timer.lock().unwrap().run();
    });

    instruction_executor_thread.join().unwrap();
    clock_thread.join().unwrap();
    timer_thread.join().unwrap();
}
