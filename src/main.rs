use std::path::{Path, PathBuf};
use std::sync::{atomic, Arc};

use structopt::StructOpt;

mod avr_emulator;

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
    let stop_program = Arc::new(atomic::AtomicBool::new(false));

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

    let avr_emulator =
        avr_emulator::AVREmulator::new(hex_dump, 1500, opt.frequency, stop_program.clone());

    let mut threads_to_join = avr_emulator.run();

    while threads_to_join.len() > 0 {
        let cur_thread = threads_to_join.remove(0);
        cur_thread.join().unwrap();
    }
}
