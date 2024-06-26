// #[path = "instruction/nop.rs"]
#[path = "instruction/add.rs"]
mod add;

mod memory;
// mod nop;

fn main() {
    println!("Hello world!");
    // let mut _inst = nop::NOP::new();
    let mut _inst2 = add::ADD::new(0);
}
