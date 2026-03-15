use std::result;

use crate::cpu::{Cpu, registers::Flag};

impl Cpu {
    pub fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => { },
            0x01 => { // LD BC, nn
                let value = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
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
            0x07 => { },
            0x08 => { // LD (nn), SP
                let addr = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
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
            0x0F => { },
            0x10 => { },
            0x11 => { // LD DE, nn
                let value = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
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
            0x17 => { },
            0x18 => { },
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
            0x1F => { },
            0x20 => { },
            0x21 => { // LD HL, nn
                let value = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
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
            0x27 => { },
            0x28 => { },
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
            0x2F => { },
            0x30 => { },
            0x31 => { // LD SP, nn
                let value = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
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
                self.bus.write(addr, self.fetch_byte());
                12
            },
            0x37 => { },
            0x38 => { },
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
            0x3F => { },

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
            0xA0 => { },
            0xA1 => { },
            0xA2 => { },
            0xA3 => { },
            0xA4 => { },
            0xA5 => { },
            0xA6 => { },
            0xA7 => { },
            0xA8 => { },
            0xA9 => { },
            0xAA => { },
            0xAB => { },
            0xAC => { },
            0xAD => { },
            0xAE => { },
            0xAF => { },
            0xB0 => { },
            0xB1 => { },
            0xB2 => { },
            0xB3 => { },
            0xB4 => { },
            0xB5 => { },
            0xB6 => { },
            0xB7 => { },
            0xB8 => { },
            0xB9 => { },
            0xBA => { },
            0xBB => { },
            0xBC => { },
            0xBD => { },
            0xBE => { },
            0xBF => { },
            0xC0 => { },
            0xC1 => { // POP BC
                self.registers.C = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.B = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xC2 => { },
            0xC3 => { },
            0xC4 => { },
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
            0xC7 => { },
            0xC8 => { },
            0xC9 => { },
            0xCA => { },
            0xCB => { }, // cbデコーダはここで呼ぶ
            0xCC => { },
            0xCD => { },
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
            0xCF => { },
            0xD0 => { },
            0xD1 => { // POP DE
                self.registers.E = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                self.registers.D = self.bus.read(self.registers.SP);
                self.registers.SP = self.registers.SP.wrapping_add(1);
                12
            },
            0xD2 => { },
            0xD3 => { },
            0xD4 => { },
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
            0xD7 => { },
            0xD8 => { },
            0xD9 => { },
            0xDA => { },
            0xDB => { },
            0xDC => { },
            0xDD => { },
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
            0xDF => { },
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
            0xE3 => { },
            0xE4 => { },
            0xE5 => { // PUSH HL
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.H);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.L);
                16
            },
            0xE6 => { },
            0xE7 => { },
            0xE8 => { // ADD SP, e
                let e = self.fetch_byte() as i8 as i16 as u16; // 負のオフセットを正しく動作させる
                let h = (self.registers.SP & 0xF) + (e & 0xF) > 0xF;
                let c = (self.registers.SP & 0xFF) + (e & 0xFF) > 0xFF;
                self.registers.SP = self.registers.SP.wrapping_add(e);
                self.registers.update_flags(Some(false), Some(false), Some(h), Some(c));
                16
            },
            0xE9 => { },
            0xEA => { // LD (nn), A
                let addr = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
                self.bus.write(addr, self.registers.A);
                16
            },
            0xEB => { },
            0xEC => { },
            0xED => { },
            0xEE => { },
            0xEF => { },
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
            0xF3 => { },
            0xF4 => { },
            0xF5 => { // PUSH AF
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.A);
                self.registers.SP = self.registers.SP.wrapping_sub(1);
                self.bus.write(self.registers.SP, self.registers.F);
                16
            },
            0xF6 => { },
            0xF7 => { },
            0xF8 => { // LD HL, SP+e
                let e = self.fetch_byte() as i8 as i16 as u16;
                self.registers.set_hl(self.registers.SP.wrapping_add(e));
                let h = (self.registers.SP & 0x0F) + (e as u16 & 0x0F) > 0x0F;
                let c = (self.registers.SP & 0xFF) + (e as u16 & 0xFF) > 0xFF;
                self.registers.update_flags(Some(false), Some(false), Some(h), Some(c));
                12
            },
            0xF9 => { self.registers.SP = self.registers.get_hl(); 8 }, // LD SP, HL
            0xFA => { // LD A, (nn)
                let addr = self.fetch_byte() as u16 | ((self.fetch_byte() as u16) << 8);
                self.registers.A = self.bus.read(addr); 
                16
            },
            0xFB => { },
            0xFC => { },
            0xFD => { },
            0xFE => { },
            0xFF => { },
            _ => panic!("未実装のオペコードです: 0x{:02X}", opcode),
        }
    }
}
