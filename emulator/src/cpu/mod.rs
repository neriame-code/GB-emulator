#![allow(unused_variables)]
#![allow(dead_code)]

use crate::bus::Bus;
pub use crate::cpu::registers::Registers;

pub mod registers;
pub mod instructions;

pub struct Cpu {
    pub registers : Registers,
    pub bus       : Bus,
    pub ime       : bool,
    pub halted    : bool,
    pub ei_pending: bool, // EI(0xFB)の実装で1命令後に1にする処理に必要
}

impl Cpu {
    pub fn fetch_byte(&mut self) -> u8 {
        let data = self.bus.read(self.registers.PC);
        self.registers.PC = self.registers.PC.wrapping_add(1);
        data
    }

    pub fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;
        (hi << 8) | lo
    }
}