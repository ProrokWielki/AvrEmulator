use std::path::{Path, PathBuf};

use structopt::StructOpt;

use registers::Registers;

pub mod instruction;
pub mod registers;

#[derive(Debug, StructOpt)]
#[structopt(name = "AVRSimulator", about = "allows running avr hex")]
struct Opt {
    #[structopt(short, long, parse(from_occurrences))]
    /// Verbose mode (-v, -vv, -vvv, etc.)
    verbose: u8,

    /// hex file to be "executed"
    #[structopt(name = "FILE", parse(from_os_str))]
    file_name: PathBuf,
}

fn get_instruction_from_address(hexdump: &bin_file::BinFile, address: usize) -> u16 {
    let offset = address * 2;

    let a = hexdump.get_value_by_address(offset).unwrap() as u16;
    let b = hexdump.get_value_by_address(offset + 1).unwrap() as u16;

    log::debug!("opcode: {:#06x}", (b << 8 | a));
    (b << 8 | a) as u16
}

fn find_instruction_from_opcode(opcode: u16, registers: &mut registers::Registers) {
    let instruction = instruction::get_instruction(opcode).unwrap();
    log::info!("instruction: {}", instruction.str());

    instruction.process(registers);
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

    log::error!("{}", file_path.display());
    let hex_dump = bin_file::BinFile::from_file(Path::new(&file_path)).unwrap();

    let mut registers = Registers::new();
    loop {
        find_instruction_from_opcode(
            get_instruction_from_address(&hex_dump, registers.pc as usize),
            &mut registers,
        );
    }
}
