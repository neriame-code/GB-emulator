use crate::bus::Bus;

// ==================== 各バス領域のReadWrite検証 ====================
#[test]
fn vram_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0x8000, mem);
    assert_eq!(bus.read(0x8000), mem, "write(0x8000) -> read(0x8000)");
}

#[test]
fn eram_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xA000, mem);
    assert_eq!(bus.read(0xA000), mem, "write(0xA000) -> read(0xA000)");
}

#[test]
fn wram_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xC000, mem);
    assert_eq!(bus.read(0xC000), mem, "write(0xC000) -> read(0xC000)");
}

#[test]
fn oam_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFE00, mem);
    assert_eq!(bus.read(0xFE00), mem, "write(0xFE00) -> read(0xFE00)");
}

#[test]
fn io_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFF00, mem);
    assert_eq!(bus.read(0xFF00), mem, "write(0xFF00) -> read(0xFF00)");
}

#[test]
fn hram_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFF80, mem);
    assert_eq!(bus.read(0xFF80), mem, "write(0xFF80) -> read(0xFF80)");
}

#[test]
fn ie_roundtrip() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFFFF, mem);
    assert_eq!(bus.read(0xFFFF), mem, "write(0xFFFF) -> read(0xFFFF)");
}

// ==================== 境界値テスト ====================
// 各領域の長さに関してもこのテストで保障(できてるはず)
// 前半は write -> 配列、後半は 配列 -> read。方向ごとに値を変えて取り違えを防ぐ
#[test]
fn boundary_vram() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0x8000, mem);
    bus.write(0x9FFF, mem_last);
    assert_eq!(bus.vram[0x0000], mem, "write(0x8000) -> vram[0x0000]");
    assert_eq!(bus.vram[0x1FFF], mem_last, "write(0x9FFF) -> vram[0x1FFF]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.vram[0x0000] = mem;
    bus.vram[0x1FFF] = mem_last;
    assert_eq!(bus.read(0x8000), mem, "vram[0x0000] -> read(0x8000)");
    assert_eq!(bus.read(0x9FFF), mem_last, "vram[0x1FFF] -> read(0x9FFF)");
}

#[test]
fn boundary_eram() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0xA000, mem);
    bus.write(0xBFFF, mem_last);
    assert_eq!(bus.eram[0x0000], mem, "write(0xA000) -> eram[0x0000]");
    assert_eq!(bus.eram[0x1FFF], mem_last, "write(0xBFFF) -> eram[0x1FFF]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.eram[0x0000] = mem;
    bus.eram[0x1FFF] = mem_last;
    assert_eq!(bus.read(0xA000), mem, "eram[0x0000] -> read(0xA000)");
    assert_eq!(bus.read(0xBFFF), mem_last, "eram[0x1FFF] -> read(0xBFFF)");
}

#[test]
fn boundary_wram() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0xC000, mem);
    bus.write(0xDFFF, mem_last);
    assert_eq!(bus.wram[0x0000], mem, "write(0xC000) -> wram[0x0000]");
    assert_eq!(bus.wram[0x1FFF], mem_last, "write(0xDFFF) -> wram[0x1FFF]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.wram[0x0000] = mem;
    bus.wram[0x1FFF] = mem_last;
    assert_eq!(bus.read(0xC000), mem, "wram[0x0000] -> read(0xC000)");
    assert_eq!(bus.read(0xDFFF), mem_last, "wram[0x1FFF] -> read(0xDFFF)");
}

#[test]
fn boundary_oam() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0xFE00, mem);
    bus.write(0xFE9F, mem_last);
    assert_eq!(bus.oam[0x00], mem, "write(0xFE00) -> oam[0x00]");
    assert_eq!(bus.oam[0x9F], mem_last, "write(0xFE9F) -> oam[0x9F]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.oam[0x00] = mem;
    bus.oam[0x9F] = mem_last;
    assert_eq!(bus.read(0xFE00), mem, "oam[0x00] -> read(0xFE00)");
    assert_eq!(bus.read(0xFE9F), mem_last, "oam[0x9F] -> read(0xFE9F)");
}

#[test]
fn boundary_io() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0xFF00, mem);
    bus.write(0xFF7F, mem_last);
    assert_eq!(bus.io[0x00], mem, "write(0xFF00) -> io[0x00]");
    assert_eq!(bus.io[0x7F], mem_last, "write(0xFF7F) -> io[0x7F]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.io[0x00] = mem;
    bus.io[0x7F] = mem_last;
    assert_eq!(bus.read(0xFF00), mem, "io[0x00] -> read(0xFF00)");
    assert_eq!(bus.read(0xFF7F), mem_last, "io[0x7F] -> read(0xFF7F)");
}

#[test]
fn boundary_hram() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    let mem_last = 0xA5u8;
    bus.write(0xFF80, mem);
    bus.write(0xFFFE, mem_last);
    assert_eq!(bus.hram[0x00], mem, "write(0xFF80) -> hram[0x00]");
    assert_eq!(bus.hram[0x7E], mem_last, "write(0xFFFE) -> hram[0x7E]");

    let mem = 0x3Cu8;
    let mem_last = 0xC3u8;
    bus.hram[0x00] = mem;
    bus.hram[0x7E] = mem_last;
    assert_eq!(bus.read(0xFF80), mem, "hram[0x00] -> read(0xFF80)");
    assert_eq!(bus.read(0xFFFE), mem_last, "hram[0x7E] -> read(0xFFFE)");
}

#[test]
fn boundary_ie() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFFFF, mem);
    assert_eq!(bus.ie, mem, "write(0xFFFF) -> ie");

    let mem = 0x3Cu8;
    bus.ie = mem;
    assert_eq!(bus.read(0xFFFF), mem, "ie -> read(0xFFFF)");
}

// ==================== EchoRAMの検証 ====================
#[test]
fn echo_ram_mirrors_wram() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xC000, mem);
    assert_eq!(bus.read(0xE000), mem, "write(0xC000) -> read(0xE000)");
}

#[test]
fn echo_ram_upper_bound() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xFDFF, mem);
    // readを経由するとEcho.writeとWRAM.readの両方を通ってしまい、原因を分析しづらくなるので直接
    assert_eq!(bus.wram[0x1DFF], mem, "write(0xFDFF) -> wram[0x1DFF]");
}

#[test]
fn echo_ram_reverse() {
    let mut bus = Bus::new();
    let mem = 0x5Au8;
    bus.write(0xDDFF, mem);
    assert_eq!(bus.read(0xFDFF), mem, "write(0xDDFF) -> read(0xFDFF)");
}

// ==================== 書き込み無視、特殊呼び出し ====================
#[test]
fn rom_area_is_read_only() {
    // TODO: MBC実装後に修正が必要
    // 事前に目印を置く。0x00のままだと「書き込みが無視された」と
    // 「ROMがそもそも全部0」を区別できない
    let mut bus = Bus::new();
    let rom_before = 0x5Au8;
    let rom_last_before = 0xA5u8;
    bus.rom[0x0000] = rom_before;
    bus.rom[0x7FFF] = rom_last_before;

    bus.write(0x0000, 0xEE);
    bus.write(0x7FFF, 0xEE);

    assert_eq!(bus.rom[0x0000], rom_before, "rom[0x0000] 不変");
    assert_eq!(bus.rom[0x7FFF], rom_last_before, "rom[0x7FFF] 不変");
}

#[test]
fn unusable_area_reads_ff() {
    // 0xFF は使用禁止領域の仕様上の戻り値なので、ここでは期待値として正しい
    let bus = Bus::new();
    assert_eq!(bus.read(0xFEA0), 0xFF, "read(0xFEA0) -> 0xFF");
    assert_eq!(bus.read(0xFEFF), 0xFF, "read(0xFEFF) -> 0xFF");
}

#[test]
fn unusable_area_write_is_ignored() {
    // 使用禁止領域の両隣(OAM末尾とIO先頭)が汚れないことを見る
    let mut bus = Bus::new();
    let oam_before = 0x5Au8;
    let io_before = 0xA5u8;
    bus.oam[0x9F] = oam_before;
    bus.io[0x00] = io_before;

    bus.write(0xFEA0, 0xEE);
    bus.write(0xFEFF, 0xEE);

    assert_eq!(bus.oam[0x9F], oam_before, "oam[0x9F] 不変");
    assert_eq!(bus.io[0x00], io_before, "io[0x00] 不変");
}

// ==================== 分離(hramとieの分離確認) ====================
#[test]
fn ie_is_separate_from_hram() {
    let mut bus = Bus::new();
    let hram_before = 0x5Au8;
    let mem = 0xA5u8;
    bus.hram[0x7E] = hram_before;
    bus.write(0xFFFF, mem);
    assert_eq!(bus.ie, mem, "write(0xFFFF) -> ie");
    assert_eq!(bus.hram[0x7E], hram_before, "hram[0x7E] 不変");

    let ie_before = 0x3Cu8;
    let mem = 0xC3u8;
    bus.ie = ie_before;
    bus.write(0xFFFE, mem);
    assert_eq!(bus.hram[0x7E], mem, "write(0xFFFE) -> hram[0x7E]");
    assert_eq!(bus.ie, ie_before, "ie 不変");
}

// ==================== 総当たり ====================
#[test]
fn all_addresses_do_not_panic() {
    // 全アドレスを走査し、panicしないことを確認する
    // そのため、assertが存在しないテストになっている
    let mut bus = Bus::new();
    for addr in 0x0000u16..=0xFFFF {
        bus.read(addr);
        bus.write(addr, 0x5A);
    }
}
