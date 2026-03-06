pub struct Bus{
    // 簡易的にMMUの内容をBusとして実装する
    pub rom : Vec<u8>,      // 32KB
    pub vram: [u8; 0x2000], // 8KB
    pub wram: [u8; 0x2000], // 8KB
    pub hram: [u8; 0x7F],   // 127byte
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        1
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        
    }
}