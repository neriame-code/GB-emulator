use crate::cpu::Cpu;
use crate::bus::Bus;

// ==================== テストで利用する定数/関数 ====================
pub(super) const PC_BEFORE: u16 = 0xC800;   // 命令列の位置（データ用アドレスと区別）
pub(super) const F_BEFORE:  u8  = 0xF0;     // 下位4bitは実機で常に0 (ZNHC -> 1111)
pub(super) const POISON:    u8  = 0xEE;     // 読まれても書かれてもいけない値
pub(super) const POISON16:  u16 = 0xEEEE;
pub(super) const REG_INIT: [u8; 8] = [0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, POISON, 0xB7];


pub(super) fn new_cpu() -> Cpu{
    let mut cpu = Cpu::new(Bus::new());
    cpu.registers.PC = PC_BEFORE;
    cpu.registers.F = F_BEFORE;
    cpu
}

#[track_caller]
pub(super) fn assert_pc_and_cycle(cpu: &Cpu, opcode: u8, expect_pc: u16, cycle: u8, expect_cycle: u8) {
    assert_eq!(cpu.registers.PC, expect_pc, "0x{:02X}: PC", opcode);
    assert_eq!(cycle, expect_cycle, "0x{:02X}: cycle", opcode);
}

pub(super) fn reg_get(cpu: &Cpu, i: u8) -> u8 {
    match i {
        0 => cpu.registers.B,
        1 => cpu.registers.C,
        2 => cpu.registers.D,
        3 => cpu.registers.E,
        4 => cpu.registers.H,
        5 => cpu.registers.L,
        6 => unreachable!(), // HL
        7 => cpu.registers.A,
        _ => unreachable!()
    }
}

pub(super) fn reg_set(cpu: &mut Cpu, i: u8, v: u8) {
    match i {
        0 => cpu.registers.B = v,
        1 => cpu.registers.C = v,
        2 => cpu.registers.D = v,
        3 => cpu.registers.E = v,
        4 => cpu.registers.H = v,
        5 => cpu.registers.L = v,
        6 => unreachable!(), // HL
        7 => cpu.registers.A = v,
        _ => unreachable!()
    }
}
