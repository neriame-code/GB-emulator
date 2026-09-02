pub struct Bus{
    // 簡易的にMMUの内容をBusとして実装する
    pub rom : Vec<u8>,      // 32KB
    pub vram: [u8; 0x2000], // 8KB
    pub eram: [u8; 0x2000], // 8KB
    pub wram: [u8; 0x2000], // 8KB
    pub oam : [u8; 0xA0],   // 160B
    pub io  : [u8; 0x80],   // 128B
    pub hram: [u8; 0x7F],   // 127B
    pub ie  : u8,           // 1B
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            // 簡易的にMMUの内容をBusとして実装する
            rom : vec![0; 0x8000],  // 32KB
            vram: [0; 0x2000],      // 8KB
            eram: [0; 0x2000],      // 8KB
            wram: [0; 0x2000],      // 8KB
            oam : [0; 0xA0],        // 160B
            io  : [0; 0x80],        // 128B
            hram: [0; 0x7F],        // 127B
            ie  : 0,                // 1B
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(addr - 0x2000 - 0xC000) as usize], // EchoRAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, // 使用禁止領域
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => (),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(addr - 0x2000 - 0xC000) as usize] = value, // EchoRAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => (), // 使用禁止領域
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value
        }
    }
}
