use crate::bus::Bus;

// ==================== 各バス領域のReadWrite検証 ====================
#[test]
fn vram_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0x8000, 0x01);
    assert_eq!(bus.read(0x8000), 0x01, "vram.read(0x8000) -> 0x01");
}

#[test]
fn eram_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xA000, 0x02);
    assert_eq!(bus.read(0xA000), 0x02, "eram.read(0xA000) -> 0x02");
}

#[test]
fn wram_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xC000, 0x03);
    assert_eq!(bus.read(0xC000), 0x03, "wram.read(0xC000) -> 0x03");
}

#[test]
fn oam_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xFE00, 0x04);
    assert_eq!(bus.read(0xFE00), 0x04, "oam.read(0xFE00) -> 0x04");
}

#[test]
fn io_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xFF00, 0x05);
    assert_eq!(bus.read(0xFF00), 0x05, "io.read(0xFF00) -> 0x05");
}

#[test]
fn hram_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xFF80, 0x06);
    assert_eq!(bus.read(0xFF80), 0x06, "hram.read(0xFF80) -> 0x06");
}

#[test]
fn ie_roundtrip() {
    let mut bus = Bus::new();
    bus.write(0xFFFF, 0x07);
    assert_eq!(bus.read(0xFFFF), 0x07, "ie.read(0xFFFF) -> 0x07");
}

// ==================== 境界値テスト ====================
// 各領域の長さに関してもこのテストで保障(できてるはず)
#[test]
fn boundary_vram() {
    let mut bus = Bus::new();
    bus.write(0x8000, 0x01);
    bus.write(0x9FFF, 0x02);
    assert_eq!(bus.vram[0x0000], 0x01, "vram[0x0000] -> 0x01");
    assert_eq!(bus.vram[0x1FFF], 0x02, "vram[0x1FFF] -> 0x02");

    bus.vram[0x0000] = 0x10;
    bus.vram[0x1FFF] = 0x20;
    assert_eq!(bus.read(0x8000), 0x10, "vram[0x0000] -> read(0x8000)");
    assert_eq!(bus.read(0x9FFF), 0x20, "vram[0x1FFF] -> read(0x9FFF)");
}

#[test]
fn boundary_eram() {
    let mut bus = Bus::new();
    bus.write(0xA000, 0x03);
    bus.write(0xBFFF, 0x04);
    assert_eq!(bus.eram[0x0000], 0x03, "eram[0x0000] -> 0x03");
    assert_eq!(bus.eram[0x1FFF], 0x04, "eram[0x1FFF] -> 0x04");

    bus.eram[0x0000] = 0x30;
    bus.eram[0x1FFF] = 0x40;
    assert_eq!(bus.read(0xA000), 0x30, "eram[0x0000] -> read(0xA000)");
    assert_eq!(bus.read(0xBFFF), 0x40, "eram[0x1FFF] -> read(0xBFFF)");
}

#[test]
fn boundary_wram() {
    let mut bus = Bus::new();
    bus.write(0xC000, 0x05);
    bus.write(0xDFFF, 0x06);
    assert_eq!(bus.wram[0x0000], 0x05, "wram[0x0000] -> 0x05");
    assert_eq!(bus.wram[0x1FFF], 0x06, "wram[0x1FFF] -> 0x06");

    bus.wram[0x0000] = 0x50;
    bus.wram[0x1FFF] = 0x60;
    assert_eq!(bus.read(0xC000), 0x50, "wram[0x0000] -> read(0xC000)");
    assert_eq!(bus.read(0xDFFF), 0x60, "wram[0x1FFF] -> read(0xDFFF)");
}

#[test]
fn boundary_oam() {
    let mut bus = Bus::new();
    bus.write(0xFE00, 0x07);
    bus.write(0xFE9F, 0x08);
    assert_eq!(bus.oam[0x00], 0x07, "oam[0x00] -> 0x07");
    assert_eq!(bus.oam[0x9F], 0x08, "oam[0x9F] -> 0x08");

    bus.oam[0x00] = 0x70;
    bus.oam[0x9F] = 0x80;
    assert_eq!(bus.read(0xFE00), 0x70, "oam[0x00] -> read(0xFE00)");
    assert_eq!(bus.read(0xFE9F), 0x80, "oam[0x9F] -> read(0xFE9F)");
}

#[test]
fn boundary_io() {
    let mut bus = Bus::new();
    bus.write(0xFF00, 0x09);
    bus.write(0xFF7F, 0x0A);
    assert_eq!(bus.io[0x00], 0x09, "io[0x00] -> 0x09");
    assert_eq!(bus.io[0x7F], 0x0A, "io[0x7F] -> 0x0A");

    bus.io[0x00] = 0x90;
    bus.io[0x7F] = 0xA0;
    assert_eq!(bus.read(0xFF00), 0x90, "io[0x00] -> read(0xFF00)");
    assert_eq!(bus.read(0xFF7F), 0xA0, "io[0x7F] -> read(0xFF7F)");
}

#[test]
fn boundary_hram() {
    let mut bus = Bus::new();
    bus.write(0xFF80, 0x0B);
    bus.write(0xFFFE, 0x0C);
    assert_eq!(bus.hram[0x00], 0x0B, "hram[0x00] -> 0x0B");
    assert_eq!(bus.hram[0x7E], 0x0C, "hram[0x7E] -> 0x0C");

    bus.hram[0x00] = 0xB0;
    bus.hram[0x7E] = 0xC0;
    assert_eq!(bus.read(0xFF80), 0xB0, "hram[0x00] -> read(0xFF80)");
    assert_eq!(bus.read(0xFFFE), 0xC0, "hram[0x7E] -> read(0xFFFE)");
}

#[test]
fn boundary_ie() {
    let mut bus = Bus::new();
    bus.write(0xFFFF, 0x0D);
    assert_eq!(bus.ie, 0x0D, "ie -> 0x0D");
    
    bus.ie = 0xD0;
    assert_eq!(bus.read(0xFFFF), 0xD0, "ie -> read(0xFFFF)");
}

// ==================== EchoRAMの検証 ====================
#[test]
fn echo_ram_mirrors_wram() {
    let mut bus = Bus::new();
    bus.write(0xC000, 0x01);
    assert_eq!(bus.read(0xE000), 0x01, "echo[0xC000] -> 0x01");
}

#[test]
fn echo_ram_upper_bound() {
    let mut bus = Bus::new();
    bus.write(0xFDFF, 0x02);
    // readを経由するとEcho.writeとWRAM.readの両方を通ってしまい、原因を分析しづらくなるので直接
    assert_eq!(bus.wram[0x1DFF], 0x02, "echo[0x1DFF] -> 0x02");
}

#[test]
fn echo_ram_reverse() {
    let mut bus = Bus::new();
    bus.write(0xDDFF, 0x03);
    assert_eq!(bus.read(0xFDFF), 0x03, "echo[0xFDFF] -> 0x03");
}

// ==================== 書き込み無視、特殊呼び出し ====================
#[test]
fn rom_area_is_read_only() {
    // TODO: MBC実装後に修正が必要
    let mut bus = Bus::new();
    bus.write(0x0000, 0x01);
    assert_eq!(bus.rom[0x0000], 0x00, "rom[0x0000] -> 0x00");

    bus.write(0x7FFF, 0x02);
    assert_eq!(bus.rom[0x7FFF], 0x00, "rom[0x7FFF] -> 0x00");
}

#[test]
fn unusable_area_reads_ff() {
    let bus = Bus::new();
    assert_eq!(bus.read(0xFEA0), 0xFF, "read(0xFEA0) -> 0xFF");
    assert_eq!(bus.read(0xFEFF), 0xFF, "read(0xFEFF) -> 0xFF");
}

#[test]
fn unusable_area_write_is_ignored() {
    let mut bus = Bus::new();
    bus.oam[0x9F] = 0x03;
    bus.io[0x00] = 0x04;
    bus.write(0xFEA0, 0x30);
    bus.write(0xFEFF, 0x40);
    assert_eq!(bus.oam[0x9F], 0x03, "oam[0x9F] -> 0x03");
    assert_eq!(bus.io[0x00], 0x04, "io[0x00] -> 0x04");
}

// ==================== 分離(hramとieの分離確認) ====================
#[test]
fn ie_is_separate_from_hram() {
    let mut bus = Bus::new();
    bus.hram[0x7E] = 0x01;
    bus.write(0xFFFF, 0x10);
    assert_eq!(bus.hram[0x7E], 0x01, "hram[0x7E] -> 0x01");
    assert_eq!(bus.ie, 0x10, "ie -> 0x10");

    bus.ie = 0x02;
    bus.write(0xFFFE, 0x20);
    assert_eq!(bus.ie, 0x02, "ie -> 0x02");
    assert_eq!(bus.hram[0x7E], 0x20, "hram[0x7E] -> 0x20");
}

// ==================== 総当たり ====================
#[test]
fn all_addresses_do_not_panic() {
    // 全アドレスを走査し、panicしないことを確認する
    // そのため、assertが存在しないテストになっている
    let mut bus = Bus::new();
    for addr in 0x0000u16..=0xFFFF {
        bus.read(addr);
        bus.write(addr, 0x01);
    }
}
