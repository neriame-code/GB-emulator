use crate::cpu::{Cpu, registers::Flag};

impl Cpu {
    // LD r,r（0x40/0x49/0x52/0x5B/0x64/0x6D/0x7F）は自己代入だが、
    // 実機に存在する命令で4サイクルを消費するため意図的にこの形で書いている
    #[allow(clippy::self_assignment)]
    pub fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => { 4 }, // NOP
            0x01 => { // LD BC, nn
                let value = self.fetch_word();
                self.registers.set_bc(value);
                12
            },
            0x02 => { // LD (BC), A
                let addr = self.registers.get_bc();
                self.bus.write(addr, self.registers.A);
                8
            },
            0x03 => { self.registers.set_bc(self.registers.get_bc().wrapping_add(1)); 8 }, // INC BC
            0x04 => { // INC B
                let result  = self.registers.B.wrapping_add(1);
                let h = self.registers.B & 0xF == 0xF;
                self.registers.B = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x05 => { // DEC B
                let result  = self.registers.B.wrapping_sub(1);
                let h = self.registers.B & 0xF == 0x0;
                self.registers.B = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x06 => { self.registers.B = self.fetch_byte(); 8 }, // LD B, n
            0x07 => { // RLCA
                let temp = self.registers.A >> 7;
                self.registers.A = self.registers.A << 1 | temp;
                self.registers.update_flags(Some(false), Some(false), Some(false), Some(temp != 0));
                4
            },
            0x08 => { // LD (nn), SP
                let addr = self.fetch_word();
                self.bus.write(addr, (self.registers.SP & 0xFF) as u8);
                self.bus.write(addr.wrapping_add(1), (self.registers.SP >> 8 & 0xFF) as u8);
                20
            },
            0x09 => { // ADD HL, BC
                let r = self.registers.get_bc();
                let h = (self.registers.get_hl() & 0xFFF) + (r & 0xFFF) > 0xFFF;
                let c = (self.registers.get_hl() as u32) + (r as u32) > 0xFFFF;
                self.registers.set_hl(self.registers.get_hl().wrapping_add(r));
                self.registers.update_flags(None, Some(false), Some(h), Some(c));
                8
            },
            0x0A => { // LD A, (BC)
                let addr = self.registers.get_bc();
                self.registers.A = self.bus.read(addr);
                8
            },
            0x0B => { self.registers.set_bc(self.registers.get_bc().wrapping_sub(1)); 8 }, // DEC BC
            0x0C => { // INC C
                let result  = self.registers.C.wrapping_add(1);
                let h = self.registers.C & 0xF == 0xF;
                self.registers.C = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x0D => { // DEC C
                let result  = self.registers.C.wrapping_sub(1);
                let h = self.registers.C & 0xF == 0x0;
                self.registers.C = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x0E => { self.registers.C = self.fetch_byte(); 8 }, // LD C, n
            0x0F => { // RRCA
                let temp = self.registers.A & 0x01;
                self.registers.A = self.registers.A >> 1 | temp << 7;
                self.registers.update_flags(Some(false), Some(false), Some(false), Some(temp != 0));
                4
            },
            0x10 => { // STOP
                self.fetch_byte();
                4
            },
            0x11 => { // LD DE, nn
                let value = self.fetch_word();
                self.registers.set_de(value);
                12
            },
            0x12 => { // LD (DE), A
                let addr = self.registers.get_de();
                self.bus.write(addr, self.registers.A);
                8
            },
            0x13 => { self.registers.set_de(self.registers.get_de().wrapping_add(1)); 8 }, // INC DE
            0x14 => { // INC D
                let result  = self.registers.D.wrapping_add(1);
                let h = self.registers.D & 0xF == 0xF;
                self.registers.D = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x15 => { // DEC D
                let result  = self.registers.D.wrapping_sub(1);
                let h = self.registers.D & 0xF == 0x0;
                self.registers.D = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x16 => { self.registers.D = self.fetch_byte(); 8 }, // LD D, n
            0x17 => { // RLA
                let temp = self.registers.A >> 7;
                self.registers.A = self.registers.A << 1 | self.registers.get_flag(Flag::C) as u8;
                self.registers.update_flags(Some(false), Some(false), Some(false), Some(temp != 0));
                4
            },
            0x18 => { // JR e
                let e = self.fetch_byte() as i8 as i16 as u16;
                self.registers.PC = self.registers.PC.wrapping_add(e);
                12
            },
            0x19 => { // ADD HL, DE
                let r = self.registers.get_de();
                let h = (self.registers.get_hl() & 0xFFF) + (r & 0xFFF) > 0xFFF;
                let c = (self.registers.get_hl() as u32) + (r as u32) > 0xFFFF;
                self.registers.set_hl(self.registers.get_hl().wrapping_add(r));
                self.registers.update_flags(None, Some(false), Some(h), Some(c));
                8
            },
            0x1A => { // LD A, (DE)
                let addr = self.registers.get_de();
                self.registers.A = self.bus.read(addr);
                8 
            },
            0x1B => { self.registers.set_de(self.registers.get_de().wrapping_sub(1)); 8 }, // DEC DE
            0x1C => { // INC E
                let result  = self.registers.E.wrapping_add(1);
                let h = self.registers.E & 0xF == 0xF;
                self.registers.E = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x1D => { // DEC E
                let result  = self.registers.E.wrapping_sub(1);
                let h = self.registers.E & 0xF == 0x0;
                self.registers.E = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x1E => { self.registers.E = self.fetch_byte(); 8 }, // LD E, n
            0x1F => { // RRA
                let temp = self.registers.A & 0x01;
                self.registers.A = self.registers.A >> 1 | (self.registers.get_flag(Flag::C) as u8) << 7;
                self.registers.update_flags(Some(false), Some(false), Some(false), Some(temp != 0));
                4
            },
            0x20 => { // JP NZ, e
                let e = self.fetch_byte() as i8 as i16 as u16;
                if !self.registers.get_flag(Flag::Z) {
                    self.registers.PC = self.registers.PC.wrapping_add(e);
                    12
                }else {
                    8
                }
            },
            0x21 => { // LD HL, nn
                let value = self.fetch_word();
                self.registers.set_hl(value);
                12
            },
            0x22 => { // LD (HL+), A
                self.bus.write(self.registers.get_hl(), self.registers.A);
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                8
            },
            0x23 => { self.registers.set_hl(self.registers.get_hl().wrapping_add(1)); 8 }, // INC HL
            0x24 => { // INC H
                let result  = self.registers.H.wrapping_add(1);
                let h = self.registers.H & 0xF == 0xF;
                self.registers.H = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x25 => { // DEC H
                let result  = self.registers.H.wrapping_sub(1);
                let h = self.registers.H & 0xF == 0x0;
                self.registers.H = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x26 => { self.registers.H = self.fetch_byte(); 8 }, // LD H, n
            0x27 => { // DAA
                let mut result = self.registers.A;
                if !self.registers.get_flag(Flag::N) {
                    if self.registers.get_flag(Flag::C) || (self.registers.A > 0x99) {
                        result = result.wrapping_add(0x60);
                        self.registers.set_flag(Flag::C, true);
                    }
                    if self.registers.get_flag(Flag::H) || (self.registers.A & 0x0F > 0x09) {
                        result = result.wrapping_add(0x06);
                    }
                }else {
                    if self.registers.get_flag(Flag::C){
                        result = result.wrapping_sub(0x60);
                        self.registers.set_flag(Flag::C, true);
                    }
                    if self.registers.get_flag(Flag::H) {
                        result = result.wrapping_sub(0x06);
                    }
                }
                self.registers.A = result;
                self.registers.update_flags(Some(result==0), None, Some(false), None);
                4
            },
            0x28 => { // JP Z, e
                let e = self.fetch_byte() as i8 as i16 as u16;
                if self.registers.get_flag(Flag::Z) {
                    self.registers.PC = self.registers.PC.wrapping_add(e);
                    12
                }else {
                    8
                }
            },
            0x29 => { // ADD HL, HL
                let r = self.registers.get_hl();
                let h = (self.registers.get_hl() & 0xFFF) + (r & 0xFFF) > 0xFFF;
                let c = (self.registers.get_hl() as u32) + (r as u32) > 0xFFFF;
                self.registers.set_hl(self.registers.get_hl().wrapping_add(r));
                self.registers.update_flags(None, Some(false), Some(h), Some(c));
                8
            },
            0x2A => { // LD A, (HL+)
                self.registers.A = self.bus.read(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1)); 
                8 
            },
            0x2B => { self.registers.set_hl(self.registers.get_hl().wrapping_sub(1)); 8 }, // DEC HL
            0x2C => { // INC L
                let result  = self.registers.L.wrapping_add(1);
                let h = self.registers.L & 0xF == 0xF;
                self.registers.L = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x2D => { // DEC L
                let result  = self.registers.L.wrapping_sub(1);
                let h = self.registers.L & 0xF == 0x0;
                self.registers.L = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x2E => { self.registers.L = self.fetch_byte(); 8 }, // LD L, n
            0x2F => { // CPL
                self.registers.A = !self.registers.A;
                self.registers.update_flags(None, Some(true), Some(true), None);
                4
            },
            0x30 => { // JP NC, e
                let e = self.fetch_byte() as i8 as i16 as u16;
                if !self.registers.get_flag(Flag::C) {
                    self.registers.PC = self.registers.PC.wrapping_add(e);
                    12
                }else {
                    8
                }
            },
            0x31 => { // LD SP, nn
                let value = self.fetch_word();
                self.registers.SP = value;
                12
            },
            0x32 => { // LD (HL-), A
                self.bus.write(self.registers.get_hl(), self.registers.A);
                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                8
            },
            0x33 => { self.registers.SP = self.registers.SP.wrapping_add(1); 8 }, // INC SP
            0x34 => { // INC (HL)
                let old_val = self.bus.read(self.registers.get_hl());
                let result  = old_val.wrapping_add(1);
                let h = old_val & 0xF == 0xF;
                self.bus.write(self.registers.get_hl(), result);
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                12
            },
            0x35 => { // DEC (HL)
                let old_val = self.bus.read(self.registers.get_hl());
                let result  = old_val.wrapping_sub(1);
                let h = old_val & 0xF == 0x0;
                self.bus.write(self.registers.get_hl(), result);
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                12
            },
            0x36 => { // LD (HL), n
                let addr = self.registers.get_hl();
                let value = self.fetch_byte();
                self.bus.write(addr, value);
                12
            },
            0x37 => { // SCF
                self.registers.update_flags(None, Some(false), Some(false), Some(true));
                4
            },
            0x38 => { // JP C, e
                let e = self.fetch_byte() as i8 as i16 as u16;
                if self.registers.get_flag(Flag::C) {
                    self.registers.PC = self.registers.PC.wrapping_add(e);
                    12
                }else {
                    8
                }
            },
            0x39 => { // ADD HL, SP
                let r = self.registers.SP;
                let h = (self.registers.get_hl() & 0xFFF) + (r & 0xFFF) > 0xFFF;
                let c = (self.registers.get_hl() as u32) + (r as u32) > 0xFFFF;
                self.registers.set_hl(self.registers.get_hl().wrapping_add(r));
                self.registers.update_flags(None, Some(false), Some(h), Some(c));
                8
            },
            0x3A => { // LD A, (HL-)
                self.registers.A = self.bus.read(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1)); 
                8 
            },
            0x3B => { self.registers.SP = self.registers.SP.wrapping_sub(1); 8 }, // DEC SP
            0x3C => { // INC A
                let result  = self.registers.A.wrapping_add(1);
                let h = self.registers.A & 0xF == 0xF;
                self.registers.A = result;
                self.registers.update_flags(Some(result==0), Some(false), Some(h), None);
                4 
            },
            0x3D => { // DEC A
                let result  = self.registers.A.wrapping_sub(1);
                let h = self.registers.A & 0xF == 0x0;
                self.registers.A = result;
                self.registers.update_flags(Some(result==0), Some(true), Some(h), None);
                4 
            },
            0x3E => { self.registers.A = self.fetch_byte(); 8 }, // LD A, n
            0x3F => { // CCF
                self.registers.update_flags(None, Some(false), Some(false), Some(!self.registers.get_flag(Flag::C)));
                4
            },
            0x40 => { self.registers.B = self.registers.B; 4 }, // 実質的にはNOP
            0x41 => { self.registers.B = self.registers.C; 4 },
            0x42 => { self.registers.B = self.registers.D; 4 },
            0x43 => { self.registers.B = self.registers.E; 4 },
            0x44 => { self.registers.B = self.registers.H; 4 },
            0x45 => { self.registers.B = self.registers.L; 4 },
            0x46 => { // LD B, (HL)
                let addr = self.registers.get_hl();
                self.registers.B = self.bus.read(addr);
                8
            },
            0x47 => { self.registers.B = self.registers.A; 4 },
            0x48 => { self.registers.C = self.registers.B; 4 },
            0x49 => { self.registers.C = self.registers.C; 4 }, // 実質的にはNOP
            0x4A => { self.registers.C = self.registers.D; 4 },
            0x4B => { self.registers.C = self.registers.E; 4 },
            0x4C => { self.registers.C = self.registers.H; 4 },
            0x4D => { self.registers.C = self.registers.L; 4 },
            0x4E => { // LD C, (HL)
                let addr = self.registers.get_hl();
                self.registers.C = self.bus.read(addr);
                8
            },
            0x4F => { self.registers.C = self.registers.A; 4 },
            0x50 => { self.registers.D = self.registers.B; 4 },
            0x51 => { self.registers.D = self.registers.C; 4 },
            0x52 => { self.registers.D = self.registers.D; 4 }, // 実質的にはNOP
            0x53 => { self.registers.D = self.registers.E; 4 },
            0x54 => { self.registers.D = self.registers.H; 4 },
            0x55 => { self.registers.D = self.registers.L; 4 },
            0x56 => { // LD D, (HL)
                let addr = self.registers.get_hl();
                self.registers.D = self.bus.read(addr);
                8
            },
            0x57 => { self.registers.D = self.registers.A; 4 },
            0x58 => { self.registers.E = self.registers.B; 4 },
            0x59 => { self.registers.E = self.registers.C; 4 },
            0x5A => { self.registers.E = self.registers.D; 4 },
            0x5B => { self.registers.E = self.registers.E; 4 }, // 実質的にはNOP
            0x5C => { self.registers.E = self.registers.H; 4 },
            0x5D => { self.registers.E = self.registers.L; 4 },
            0x5E => { // LD E, (HL)
                let addr = self.registers.get_hl();
                self.registers.E = self.bus.read(addr);
                8
            },
            0x5F => { self.registers.E = self.registers.A; 4 },
            0x60 => { self.registers.H = self.registers.B; 4 },
            0x61 => { self.registers.H = self.registers.C; 4 },
            0x62 => { self.registers.H = self.registers.D; 4 },
            0x63 => { self.registers.H = self.registers.E; 4 },
            0x64 => { self.registers.H = self.registers.H; 4 }, // 実質的にはNOP
            0x65 => { self.registers.H = self.registers.L; 4 },
            0x66 => { // LD H, (HL)
                let addr = self.registers.get_hl();
                self.registers.H = self.bus.read(addr);
                8
            },
            0x67 => { self.registers.H = self.registers.A; 4 },
            0x68 => { self.registers.L = self.registers.B; 4 },
            0x69 => { self.registers.L = self.registers.C; 4 },
            0x6A => { self.registers.L = self.registers.D; 4 },
            0x6B => { self.registers.L = self.registers.E; 4 },
            0x6C => { self.registers.L = self.registers.H; 4 },
            0x6D => { self.registers.L = self.registers.L; 4 }, // 実質的にはNOP
            0x6E => { // LD L, (HL)
                let addr = self.registers.get_hl();
                self.registers.L = self.bus.read(addr);
                8
            },
            0x6F => { self.registers.L = self.registers.A; 4 },
            0x70 => { // LD (HL), B
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.B);
                8
            },
            0x71 => { // LD (HL), C
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.C);
                8
            },
            0x72 => { // LD (HL), D
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.D);
                8
            },
            0x73 => { // LD (HL), E
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.E);
                8
            },
            0x74 => { // LD (HL), H
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.H);
                8
            },
            0x75 => { // LD (HL), L
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.L);
                8
            },
            0x76 => { self.halted = true; 4 }, // HALT
            0x77 => { // LD (HL), A
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.registers.A);
                8
            },
            0x78 => { self.registers.A = self.registers.B; 4 },
            0x79 => { self.registers.A = self.registers.C; 4 },
            0x7A => { self.registers.A = self.registers.D; 4 },
            0x7B => { self.registers.A = self.registers.E; 4 },
            0x7C => { self.registers.A = self.registers.H; 4 },
            0x7D => { self.registers.A = self.registers.L; 4 },
            0x7E => { // LD r, (HL)
                let addr = self.registers.get_hl();
                self.registers.A = self.bus.read(addr);
                8
            },
            0x7F => { self.registers.A = self.registers.A; 4 }, // 実質的にはNOP
            0x80 => { // ADD A, B
                let r = self.registers.B;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x81 => { // ADD A, C
                let r = self.registers.C;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x82 => { // ADD A, D
                let r = self.registers.D;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x83 => { // ADD A, E
                let r = self.registers.E;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x84 => { // ADD A, H
                let r = self.registers.H;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x85 => { // ADD A, L
                let r = self.registers.L;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x86 => { // ADD A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                8
            },
            0x87 => { // ADD A, A
                let r = self.registers.A;
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x88 => { // ADC A, B
                let r = self.registers.B;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x89 => { // ADC A, C
                let r = self.registers.C;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x8A => { // ADC A, D
                let r = self.registers.D;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x8B => { // ADC A, E
                let r = self.registers.E;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x8C => { // ADC A, H
                let r = self.registers.H;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x8D => { // ADC A, L
                let r = self.registers.L;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x8E => { // ADC A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                8
            },
            0x8F => { // ADC A, A
                let r = self.registers.A;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                4
            },
            0x90 => { // SUB A, B
                let r = self.registers.B;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x91 => { // SUB A, C
                let r = self.registers.C;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x92 => { // SUB A, D
                let r = self.registers.D;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x93 => { // SUB A, E
                let r = self.registers.E;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x94 => { // SUB A, H
                let r = self.registers.H;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x95 => { // SUB A, L
                let r = self.registers.L;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x96 => { // SUB A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0x97 => { // SUB A, A
                let r = self.registers.A;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x98 => { // SBC A, B
                let r = self.registers.B;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x99 => { // SBC A, C
                let r = self.registers.C;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x9A => { // SBC A, D
                let r = self.registers.D;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x9B => { // SBC A, E
                let r = self.registers.E;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x9C => { // SBC A, H
                let r = self.registers.H;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x9D => { // SBC A, L
                let r = self.registers.L;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0x9E => { // SBC A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0x9F => { // SBC A, A
                let r = self.registers.A;
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xA0 => { // AND A, B
                let r = self.registers.B;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA1 => { // AND A, C
                let r = self.registers.C;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA2 => { // AND A, D
                let r = self.registers.D;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA3 => { // AND A, E
                let r = self.registers.E;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA4 => { // AND A, H
                let r = self.registers.H;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA5 => { // AND A, L
                let r = self.registers.L;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA6 => { // AND A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                8
            },
            0xA7 => { // AND A, A
                let r = self.registers.A;
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                4
            },
            0xA8 => { // XOR A, B
                let r = self.registers.B;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xA9 => { // XOR A, C
                let r = self.registers.C;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xAA => { // XOR A, D
                let r = self.registers.D;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xAB => { // XOR A, E
                let r = self.registers.E;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xAC => { // XOR A, H
                let r = self.registers.H;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xAD => { // XOR A, L
                let r = self.registers.L;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xAE => { // XOR A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                8
            },
            0xAF => { // XOR A, A
                let r = self.registers.A;
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB0 => { // OR A, B
                let r = self.registers.B;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB1 => { // OR A, C
                let r = self.registers.C;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB2 => { // OR A, D
                let r = self.registers.D;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB3 => { // OR A, E
                let r = self.registers.E;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB4 => { // OR A, H
                let r = self.registers.H;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB5 => { // OR A, L
                let r = self.registers.L;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB6 => { // OR A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                8
            },
            0xB7 => { // OR A, A
                let r = self.registers.A;
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                4
            },
            0xB8 => { // CP A, B
                let r = self.registers.B;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xB9 => { // CP A, C
                let r = self.registers.C;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xBA => { // CP A, D
                let r = self.registers.D;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xBB => { // CP A, E
                let r = self.registers.E;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xBC => { // CP A, H
                let r = self.registers.H;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xBD => { // CP A, L
                let r = self.registers.L;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xBE => { // CP A, (HL)
                let r = self.bus.read(self.registers.get_hl());
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0xBF => { // CP A, A
                let r = self.registers.A;
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                4
            },
            0xC0 => { // RET NZ
                if !self.registers.get_flag(Flag::Z) {
                    let lo = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    let hi = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                    20
                }else {
                    8
                }
            },
            0xC1 => { // POP BC
                self.registers.C = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.B = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xC2 => { // JP NZ, nn
                let nn = self.fetch_word();
                if !self.registers.get_flag(Flag::Z) {
                    self.registers.PC = nn;
                    16
                }else {
                    12
                }
            },
            0xC3 => { self.registers.PC = self.fetch_word(); 16 }, // JP nn
            0xC4 => { // CALL NZ, nn
                let nn = self.fetch_word();
                if !self.registers.get_flag(Flag::Z) {
                    let ret_addr = self.registers.PC;
                    let hi = (ret_addr >> 8 & 0xFF) as u8;
                    let lo = (ret_addr & 0xFF) as u8;
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, hi);
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, lo);
                    self.registers.PC = nn;
                    24
                }else {
                    12
                }
            },
            0xC5 => { // PUSH BC
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.B);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.C);
                16
            },
            0xC6 => { // ADD A, n
                let r = self.fetch_byte();
                let result = self.registers.A.wrapping_add(r);
                let h = (self.registers.A & 0xF) + (r & 0xF) > 0xF;
                let c = (self.registers.A as u16) + (r as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                8
            },
            0xC7 => { // RST 0x00
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0000;
                16
            },
            0xC8 => { // RET Z
                if self.registers.get_flag(Flag::Z) {
                    let lo = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    let hi = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                    20
                }else {
                    8
                }
            },
            0xC9 => { // RET
                let lo = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                let hi = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                16
            },
            0xCA => { // JP Z, nn
                let nn = self.fetch_word();
                if self.registers.get_flag(Flag::Z) {
                    self.registers.PC = nn;
                    16
                }else {
                    12
                }
            },
            0xCB => { // cbデコーダはここで呼ぶ
                let opecode = self.fetch_byte();
                self.prefix_execute(opecode)
            },
            0xCC => { // CALL Z, nn
                let nn = self.fetch_word();
                if self.registers.get_flag(Flag::Z) {
                    let ret_addr = self.registers.PC;
                    let hi = (ret_addr >> 8 & 0xFF) as u8;
                    let lo = (ret_addr & 0xFF) as u8;
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, hi);
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, lo);
                    self.registers.PC = nn;
                    24
                }else {
                    12
                }
            },
            0xCD => { // CALL nn
                let nn = self.fetch_word(); // PCがCALL命令の次の命令を指す
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                // PCをリターンアドレスとしてスタックに積む
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = nn; // PCをnnに書き換えてジャンプ
                24
            },
            0xCE => { // ADC A, n
                let r = self.fetch_byte();
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_add(r).wrapping_add(carry);
                let h = (self.registers.A & 0xF) + (r & 0xF) + carry > 0xF;
                let c = (self.registers.A as u16) + (r as u16) + (carry as u16) > 0xFF;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(h), Some(c));
                8
            },
            0xCF => { // RST 0x08
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0008;
                16
            },
            0xD0 => { // RET NC
                if !self.registers.get_flag(Flag::C) {
                    let lo = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    let hi = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                    20
                }else {
                    8
                }
            },
            0xD1 => { // POP DE
                self.registers.E = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.D = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xD2 => { // JP NC, nn
                let nn = self.fetch_word();
                if !self.registers.get_flag(Flag::C) {
                    self.registers.PC = nn;
                    16
                }else {
                    12
                }
            },
            // 0xD3 => { },
            0xD4 => { // CALL NC, nn
                let nn = self.fetch_word();
                if !self.registers.get_flag(Flag::C) {
                    let ret_addr = self.registers.PC;
                    let hi = (ret_addr >> 8 & 0xFF) as u8;
                    let lo = (ret_addr & 0xFF) as u8;
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, hi);
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, lo);
                    self.registers.PC = nn;
                    24
                }else {
                    12
                }
            },
            0xD5 => { // PUSH DE
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.D);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.E);
                16
            },
            0xD6 => { // SUB A, n
                let r = self.fetch_byte();
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0xD7 => { // RST 0x10
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0010;
                16
            },
            0xD8 => { // RET C
                if self.registers.get_flag(Flag::C) {
                    let lo = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    let hi = self.bus.read(self.registers.SP);
                    self.registers.SP = self.registers.SP.wrapping_add(1);
                    self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                    20
                }else {
                    8
                }
            },
            0xD9 => { // RETI
                let lo = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                let hi = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.PC = ((hi as u16) << 8) | (lo as u16);
                self.ime = true;
                16
            },
            0xDA => { // JP C, nn
                let nn = self.fetch_word();
                if self.registers.get_flag(Flag::C) {
                    self.registers.PC = nn;
                    16
                }else {
                    12
                }
            },
            // 0xDB => { },
            0xDC => { // CALL C, nn
                let nn = self.fetch_word();
                if self.registers.get_flag(Flag::C) {
                    let ret_addr = self.registers.PC;
                    let hi = (ret_addr >> 8 & 0xFF) as u8;
                    let lo = (ret_addr & 0xFF) as u8;
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, hi);
                    self.registers.SP = self.registers.SP.wrapping_sub(1);
                    self.bus.write(self.registers.SP, lo);
                    self.registers.PC = nn;
                    24
                }else {
                    12
                }
            },
            // 0xDD => { },
            0xDE => { // SBC A, n
                let r = self.fetch_byte();
                let carry = self.registers.get_flag(Flag::C) as u8;
                let result = self.registers.A.wrapping_sub(r).wrapping_sub(carry);
                let h = (self.registers.A & 0xF) < (r & 0xF) + carry; // 加算によるオーバーフロー回避
                let c = (self.registers.A as u16) < (r as u16) + (carry as u16);
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0xDF => { // RST 0x18
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0018;
                16
            },
            0xE0 => { // LDH (n), A
                let addr = 0xFF00 | self.fetch_byte() as u16;
                self.bus.write(addr, self.registers.A);
                12
            },
            0xE1 => { // POP HL
                self.registers.L = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.H = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xE2 => { // LDH (C), A
                let addr = 0xFF00 | self.registers.C as u16;
                self.bus.write(addr, self.registers.A);
                8
            },
            // 0xE3 => { },
            // 0xE4 => { },
            0xE5 => { // PUSH HL
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.H);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.L);
                16
            },
            0xE6 => { // AND A, n
                let r = self.fetch_byte();
                let result = self.registers.A & r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(true), Some(false));
                8
            },
            0xE7 => { // RST 0x20
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0020;
                16
            },
            0xE8 => { // ADD SP, e
                let e = self.fetch_byte() as i8 as i16 as u16; // 負のオフセットを正しく動作させる
                let h = (self.registers.SP & 0xF) + (e & 0xF) > 0xF;
                let c = (self.registers.SP & 0xFF) + (e & 0xFF) > 0xFF;
                self.registers.SP = self.registers.SP.wrapping_add(e);
                self.registers.update_flags(Some(false), Some(false), Some(h), Some(c));
                16
            },
            0xE9 => { self.registers.PC = self.registers.get_hl(); 4 }, // JP (HL)
            0xEA => { // LD (nn), A
                let addr = self.fetch_word();
                self.bus.write(addr, self.registers.A);
                16
            },
            // 0xEB => { },
            // 0xEC => { },
            // 0xED => { },
            0xEE => { // XOR A, n
                let r = self.fetch_byte();
                let result = self.registers.A ^ r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                8
            },
            0xEF => { // RST 0x28
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0028;
                16
            },
            0xF0 => { // LDH A, (n)
                let addr = 0xFF00 | self.fetch_byte() as u16;
                self.registers.A = self.bus.read(addr);
                12
            },
            0xF1 => { // POP AF
                self.registers.F = self.bus.read(self.registers.SP) & 0xF0;
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.A = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xF2 => { // LD A, (C)
                let addr = 0xFF00 | self.registers.C as u16;
                self.registers.A = self.bus.read(addr);
                8
            },
            0xF3 => { self.ime = false; 4 }, // DI
            // 0xF4 => { },
            0xF5 => { // PUSH AF
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.A);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.F);
                16
            },
            0xF6 => { // OR A, n
                let r = self.fetch_byte();
                let result = self.registers.A | r;
                self.registers.A = result;
                self.registers.update_flags(Some(result == 0), Some(false), Some(false), Some(false));
                8
            },
            0xF7 => { // RST 0x30
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0030;
                16
            },
            0xF8 => { // LD HL, SP+e
                let e = self.fetch_byte() as i8 as i16 as u16;
                self.registers.set_hl(self.registers.SP.wrapping_add(e));
                let h = (self.registers.SP & 0x0F) + (e & 0x0F) > 0x0F;
                let c = (self.registers.SP & 0xFF) + (e & 0xFF) > 0xFF;
                self.registers.update_flags(Some(false), Some(false), Some(h), Some(c));
                12
            },
            0xF9 => { self.registers.SP = self.registers.get_hl(); 8 }, // LD SP, HL
            0xFA => { // LD A, (nn)
                let addr = self.fetch_word();
                self.registers.A = self.bus.read(addr); 
                16
            },
            0xFB => { self.ei_pending = true; 4 }, // EI
            // 0xFC => { },
            // 0xFD => { },
            0xFE => { // CP A, n
                let r = self.fetch_byte();
                let result = self.registers.A.wrapping_sub(r);
                let h = (self.registers.A & 0xF) < (r & 0xF);
                let c = self.registers.A < r;
                self.registers.update_flags(Some(result == 0), Some(true), Some(h), Some(c));
                8
            },
            0xFF => { // RST 0x38
                let ret_addr = self.registers.PC;
                let hi = (ret_addr >> 8 & 0xFF) as u8;
                let lo = (ret_addr & 0xFF) as u8;
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, hi);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, lo);
                self.registers.PC = 0x0038;
                16
            },
            _ => panic!("未実装のオペコードです: 0x{:02X}", opcode),
        }
    }
}
