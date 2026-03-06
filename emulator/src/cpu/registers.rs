#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

pub struct Registers {
    pub(crate) A:  u8,  pub(crate) F:  u8,
    pub(crate) B:  u8,  pub(crate) C:  u8,
    pub(crate) D:  u8,  pub(crate) E:  u8,
    pub(crate) H:  u8,  pub(crate) L:  u8,
    pub(crate) SP: u16, pub(crate) PC: u16,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Flag { Z = 0x80, N = 0x40, H = 0x20, C = 0x10, }

impl Registers {
    // === フラグレジスタ操作用メソッド ===
    pub fn get_flag(&self, flag: Flag) -> bool { (self.F & (flag as u8)) != 0 }
    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value { self.F |= flag as u8; }
        else     { self.F &= !(flag as u8); }
    }
    // 4つのフラグ全てが確定する演算命令でフラグを一括更新する
    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        let mut f = 0u8; // 初期値に対して必要なフラグを立ててからビットを立てる
        if z { f |= Flag::Z as u8; }
        if n { f |= Flag::N as u8; }
        if h { f |= Flag::H as u8; }
        if c { f |= Flag::C as u8; }
        self.F = f & 0xF0;
    }
    // 一部のフラグ状態を維持しながらフラグを更新する
    pub fn update_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        let mut f = self.F;
        if let Some(v) = z { if v { f |= Flag::Z as u8; } else { f &= !(Flag::Z as u8); } }
        if let Some(v) = n { if v { f |= Flag::N as u8; } else { f &= !(Flag::N as u8); } }
        if let Some(v) = h { if v { f |= Flag::H as u8; } else { f &= !(Flag::H as u8); } }
        if let Some(v) = c { if v { f |= Flag::C as u8; } else { f &= !(Flag::C as u8); } }
        self.F = f & 0xF0;
    }

    // === 16bitレジスタ用メソッド ===
    // getterとして16bitへ変換
    // setterとして分割して代入
    fn get_pair(high: u8, low: u8) -> u16 { ((high as u16) << 8) | (low as u16) }
    fn set_pair(high: &mut u8, low: &mut u8, value: u16) {
        *high = (value >> 8) as u8;
        *low = value as u8;
    }

    pub fn get_af(&self) -> u16 { Self::get_pair(self.A, self.F) }
    // Fレジスタ0xF0 (0b1111_0000) でマスクして、下位4ビットを強制的に0にする
    pub fn set_af(&mut self, value: u16) { Self::set_pair(&mut self.A, &mut self.F, value & 0xFFF0); }

    pub fn get_bc(&self) -> u16 { Self::get_pair(self.B, self.C) }
    pub fn set_bc(&mut self, value: u16) { Self::set_pair(&mut self.B, &mut self.C, value); }

    pub fn get_de(&self) -> u16 { Self::get_pair(self.D, self.E) }
    pub fn set_de(&mut self, value: u16) { Self::set_pair(&mut self.D, &mut self.E, value); }

    pub fn get_hl(&self) -> u16 { Self::get_pair(self.H, self.L) }
    pub fn set_hl(&mut self, value: u16) { Self::set_pair(&mut self.H, &mut self.L, value); }
}
