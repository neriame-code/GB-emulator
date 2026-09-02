mod cpu;
mod mmu;
mod ppu;
mod timer;
mod input;

mod bus;
mod rom;
mod emulator;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)] mod tests;