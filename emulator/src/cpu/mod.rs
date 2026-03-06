#![allow(unused_variables)]
#![allow(dead_code)]

use crate::bus::Bus;
pub use crate::cpu::registers::Registers;

pub mod registers;
pub mod instructions;

pub struct Cpu {
    pub registers: Registers,
    pub bus      : Bus
}

impl Cpu {
    pub fn fetch_byte(&mut self) -> u8 {
        let data = self.bus.read(self.registers.PC);
        self.registers.PC = self.registers.PC.wrapping_add(1);
        data
    } 
}