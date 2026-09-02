use crate::cpu::Cpu;

impl Cpu {
    pub fn prefix_execute(&mut self, opcode: u8) -> u8 {
        todo!("CB命令 未実装: 0x{:02X}", opcode)
    }
}
