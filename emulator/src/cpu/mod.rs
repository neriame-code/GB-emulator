#![allow(unused_variables)]
#![allow(dead_code)]

use crate::cpu::registers::Registers;

pub mod registers;
pub mod instructions;

pub struct Cpu {
    registers: Registers,
}