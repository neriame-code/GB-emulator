use crate::cpu::{Cpu, registers::Flag};

impl Cpu {
    pub fn prefix_execute(&mut self, opcode: u8) -> u8 {
        // opcode 0b XX YYY ZZZ
        // XX  = 範囲ごとに演算方法が異なる
        // YYY = 演算の種類 or ビット番号 n
        // ZZZ = B,C,D,E,H,L,(HL),A (target)
        let xx = opcode >> 6;
        let yyy = (opcode >> 3) & 7;
        let zzz = opcode & 7;

        match xx {
            0 => { // opcode = 0x00〜0x3F
                match yyy {
                    0 => self.rlc(zzz),
                    1 => self.rrc(zzz),
                    2 => self.rl(zzz),
                    3 => self.rr(zzz),
                    4 => self.sla(zzz),
                    5 => self.sra(zzz),
                    6 => self.swap(zzz),
                    7 => self.srl(zzz),
                    _ => unreachable!()
                }
                // 戻り値はどの処理でも共通
                if zzz == 6 { 16 } else { 8 }
            },
            1 => self.bit_n(yyy, zzz), // BIT: opcode = 0x40〜0x7F
            2 => self.res_n(yyy, zzz), // RES: opcode = 0x80〜0xBF
            3 => self.set_n(yyy, zzz), // SET: opcode = 0xC0〜0xFF
            _ => unreachable!()
        }
    }

    fn read_target(&mut self, target: u8) -> u8 {
        match target {
            0 => self.registers.B,
            1 => self.registers.C,
            2 => self.registers.D,
            3 => self.registers.E,
            4 => self.registers.H,
            5 => self.registers.L,
            6 => self.bus.read(self.registers.get_hl()),
            7 => self.registers.A,
            _ => unreachable!()
        }
    }

    fn write_target(&mut self, target: u8, value: u8) {
        match target {
            0 => { self.registers.B = value; },
            1 => { self.registers.C = value; },
            2 => { self.registers.D = value; },
            3 => { self.registers.E = value; },
            4 => { self.registers.H = value; },
            5 => { self.registers.L = value; },
            6 => { self.bus.write(self.registers.get_hl(), value); },
            7 => { self.registers.A = value; },
            _ => unreachable!()
        }
    }

    // === シフト/ローテート8種 ===
    fn rlc(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value >> 7;
        value = (value << 1) | carry; // 
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn rrc(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value & 0x01;
        value = (value >> 1) | (carry << 7);
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn rl(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value >> 7;
        value = (value << 1) | (self.registers.get_flag(Flag::C) as u8);
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn rr(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value & 0x01;
        value = (value >> 1) | ((self.registers.get_flag(Flag::C) as u8) << 7);
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn sla(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value >> 7;
        value <<= 1; // bit0が0になる
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn sra(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value & 0x01;
        value = (value >> 1) | (value & 0x80); // bit7に元のbit7が入る
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    fn swap(&mut self, target: u8){
        // 上位4bitと下位4bitの入れ替え
        let mut value = self.read_target(target);
        value = value.rotate_left(4); // (value << 4) | (value >> 4)
        self.registers.set_flags(value == 0, false, false, false);
        self.write_target(target, value);
    }

    fn srl(&mut self, target: u8){
        let mut value = self.read_target(target);
        let carry = value & 0x01;
        value >>= 1; // bit7が0になる
        self.registers.set_flags(value == 0, false, false, carry != 0);
        self.write_target(target, value);
    }

    // === その他 ===
    fn bit_n(&mut self, bit:u8, target: u8) -> u8 {
        let value = self.read_target(target);
        let result = value & (1 << bit);
        self.registers.update_flags(Some(result == 0), Some(false), Some(true), None);
        if target == 6 { 12 } else { 8 }
    }

    fn res_n(&mut self, bit:u8, target: u8) -> u8 {
        let value = self.read_target(target);
        let result = value & !(1 << bit);
        self.write_target(target, result);
        if target == 6 { 16 } else { 8 }
    }

    fn set_n(&mut self, bit:u8, target: u8) -> u8 {
        let value = self.read_target(target);
        let result = value | (1 << bit);
        self.write_target(target, result);
        if target == 6 { 16 } else { 8 }
    }
}
