use crate::cpu::Cpu;

impl Cpu {
    pub fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => { },
            0x01 => { },
            0x02 => { // LD (BC), A
                let addr = self.registers.get_bc();
                self.bus.write(addr, self.registers.A);
                8
            },
            0x03 => { },
            0x04 => { },
            0x05 => { },
            0x06 => { self.registers.B = self.fetch_byte(); 8 }, // LD B, n
            0x07 => { },
            0x08 => { },
            0x09 => { },
            0x0A => { // LD A, (BC)
                let addr = self.registers.get_bc();
                self.registers.A = self.bus.read(addr);
                8
            },
            0x0B => { },
            0x0C => { },
            0x0D => { },
            0x0E => { self.registers.C = self.fetch_byte(); 8 }, // LD C, n
            0x0F => { },
            0x10 => { },
            0x11 => { },
            0x12 => { // LD (DE), A
                let addr = self.registers.get_de();
                self.bus.write(addr, self.registers.A);
                8
            },
            0x13 => { },
            0x14 => { },
            0x15 => { },
            0x16 => { self.registers.D = self.fetch_byte(); 8 }, // LD D, n
            0x17 => { },
            0x18 => { },
            0x19 => { },
            0x1A => { // LD A, (DE)
                let addr = self.registers.get_de();
                self.registers.A = self.bus.read(addr);
                8 
            },
            0x1B => { },
            0x1C => { },
            0x1D => { },
            0x1E => { self.registers.E = self.fetch_byte(); 8 }, // LD E, n
            0x1F => { },
            0x20 => { },
            0x21 => { },
            0x22 => { // LD (HL+), A
                let addr = self.registers.get_hl();
                self.bus.write(addr.wrapping_add(1), self.registers.A);
                8
            },
            0x23 => { },
            0x24 => { },
            0x25 => { },
            0x26 => { self.registers.H = self.fetch_byte(); 8 }, // LD H, n
            0x27 => { },
            0x28 => { },
            0x29 => { },
            0x2A => { // LD A, (HL+)
                let addr = self.registers.get_hl().wrapping_add(1);
                self.registers.A = self.bus.read(addr); 
                8 
            },
            0x2B => { },
            0x2C => { },
            0x2D => { },
            0x2E => { self.registers.L = self.fetch_byte(); 8 }, // LD L, n
            0x2F => { },
            0x30 => { },
            0x31 => { },
            0x32 => { // LD (HL-), A
                let addr = self.registers.get_hl();
                self.bus.write(addr.wrapping_sub(1), self.registers.A);
                8
            },
            0x33 => { },
            0x34 => { },
            0x35 => { },
            0x36 => { // LD (HL), n
                let addr = self.registers.get_hl();
                self.bus.write(addr, self.fetch_byte());
                12
            },
            0x37 => { },
            0x38 => { },
            0x39 => { },
            0x3A => { // LD A, (HL-)
                let addr = self.registers.get_hl().wrapping_sub(1); 
                self.registers.A = self.bus.read(addr); 
                8 
            },
            0x3B => { },
            0x3C => { },
            0x3D => { },
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

            0x80 => { },
            0x81 => { },
            0x82 => { },
            0x83 => { },
            0x84 => { },
            0x85 => { },
            0x86 => { },
            0x87 => { },
            0x88 => { },
            0x89 => { },
            0x8A => { },
            0x8B => { },
            0x8C => { },
            0x8D => { },
            0x8E => { },
            0x8F => { },
            0x90 => { },
            0x91 => { },
            0x92 => { },
            0x93 => { },
            0x94 => { },
            0x95 => { },
            0x96 => { },
            0x97 => { },
            0x98 => { },
            0x99 => { },
            0x9A => { },
            0x9B => { },
            0x9C => { },
            0x9D => { },
            0x9E => { },
            0x9F => { },
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
            0xC1 => { },
            0xC2 => { },
            0xC3 => { },
            0xC4 => { },
            0xC5 => { },
            0xC6 => { },
            0xC7 => { },
            0xC8 => { },
            0xC9 => { },
            0xCA => { },
            0xCB => { }, // cbデコーダはここで呼ぶ
            0xCC => { },
            0xCD => { },
            0xCE => { },
            0xCF => { },
            0xD0 => { },
            0xD1 => { },
            0xD2 => { },
            0xD3 => { },
            0xD4 => { },
            0xD5 => { },
            0xD6 => { },
            0xD7 => { },
            0xD8 => { },
            0xD9 => { },
            0xDA => { },
            0xDB => { },
            0xDC => { },
            0xDD => { },
            0xDE => { },
            0xDF => { },
            0xE0 => { // LDH (n), A
                let addr = 0xFF00 | self.fetch_byte() as u16;
                self.bus.write(addr, self.registers.A);
                12
            },
            0xE1 => { },
            0xE2 => { // LDH (C), A
                let addr = 0xFF00 | self.registers.C as u16;
                self.bus.write(addr, self.registers.A);
                8
            },
            0xE3 => { },
            0xE4 => { },
            0xE5 => { },
            0xE6 => { },
            0xE7 => { },
            0xE8 => { },
            0xE9 => { },
            0xEA => { // LD (nn), A
                let addr = ((self.fetch_byte() as u16) << 8) | self.fetch_byte() as u16;
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
            0xF1 => { },
            0xF2 => { // LD A, (C)
                let addr = 0xFF00 | self.registers.C as u16;
                self.registers.A = self.bus.read(addr);
                8
            },
            0xF3 => { },
            0xF4 => { },
            0xF5 => { },
            0xF6 => { },
            0xF7 => { },
            0xF8 => { },
            0xF9 => { },
            0xFA => { // LD A, (nn)
                let addr = ((self.fetch_byte() as u16) << 8) | self.fetch_byte() as u16;
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
