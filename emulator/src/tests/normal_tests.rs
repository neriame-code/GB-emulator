use crate::cpu::{Cpu, registers::Flag};
use crate::bus::Bus;

// ==================== テストで利用する定数/関数 ====================
const PC_BEFORE: u16 = 0xC800;   // 命令列の位置（データ用アドレスと区別）
const F_BEFORE:  u8  = 0xF0;     // 下位4bitは実機で常に0 (ZNHC -> 1111)
const POISON:    u8  = 0xEE;     // 読まれても書かれてもいけない値
const POISON16:  u16 = 0xEEEE;
const REG_INIT: [u8; 8] = [0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, POISON, 0xB7];


fn new_cpu() -> Cpu{
    let mut cpu = Cpu::new(Bus::new());
    cpu.registers.PC = PC_BEFORE;
    cpu.registers.F = F_BEFORE;
    cpu
}

#[track_caller]
fn assert_pc_and_cycle(cpu: &Cpu, opcode: u8, expect_pc: u16, cycle: u8, expect_cycle: u8) {
    assert_eq!(cpu.registers.PC, expect_pc, "0x{:02X}: PC", opcode);
    assert_eq!(cycle, expect_cycle, "0x{:02X}: cycle", opcode);
}

fn reg_get(cpu: &Cpu, i: u8) -> u8 {
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

fn reg_set(cpu: &mut Cpu, i: u8, v: u8) {
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

// ==================== 基本制御系 ====================
#[ignore]
#[test]
fn nop_costs_4_cycles() {
    todo!()
}

#[test]
fn stop_consumes_one_operand_byte() {
    let mut cpu = new_cpu();
    let opcode = 0x10u8; // STOP
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, F_BEFORE, "F 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE + 1, cycle, 4);
}

#[ignore]
#[test]
fn halt_sets_halted_flag() {
    todo!()
}

#[ignore]
#[test]
fn di_clears_ime() {
    todo!()
}

#[ignore]
#[test]
fn ei_sets_pending_not_ime() {
    todo!()
}

#[test]
#[should_panic(expected = "未定義のオペコードです")]
fn undefined_opcode_panics() {
    // 未定義は次の通り。代表値のみをテストする。パニックのテストなのでAssertは書かない
    // 0xD3  0xDB  0xDD  0xE3  0xE4  0xEB  0xEC  0xED  0xF4  0xFC  0xFD
    let mut cpu = new_cpu();
    let opcode = 0xDD;
    cpu.execute(opcode);
}

// ==================== 8bitロード ====================
#[test]
fn ld_r_n_loads_immediate() {
    // LD命令の代表として LD B, n を用いる
    let mut cpu = new_cpu();
    let opcode = 0x06u8; // LD B, n
    let n = 0x5Au8;
    let b_before = 0x3Cu8;
    let f_before = 0xF0u8;
    cpu.bus.write(PC_BEFORE, n);
    cpu.registers.B = b_before;
    cpu.registers.F = f_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.B, n, "n -> B");
    assert_eq!(cpu.registers.F, f_before, "F 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE + 1, cycle, 8);
}

#[ignore]
#[test]
fn ld_r_n_all_registers() {
    todo!()
}

#[ignore]
#[test]
fn ld_r_r_copies_source() {
    todo!()
}

#[test]
fn ld_r_r_all_opcodes() {
    // HLに関するテストは別で行っている
    // index: 0=B 1=C 2=D 3=E 4=H 5=L 6=(HL) 7=A
    // init は下位ニブルが出所を表す。index 6 は触らないので「あってはならない」印を置く

    let f_before = 0xF0u8;
    let mut count = 0;

    // opcode = 0b01_ddd_sss
    // ただしddd == 6とsss == 6はやらなくてOK(HALT関連)
    for opcode in 0x40..=0x7Fu8 {
        let mut cpu = new_cpu();
        let ddd = (opcode >> 3) & 7; // 転送先
        let sss = opcode & 7;        // 転送元
        if ddd == 6 || sss == 6 { continue; }

        for i in 0..8u8 {
            if i == 6 { continue; }
            reg_set(&mut cpu, i, REG_INIT[i as usize]);
        }

        let cycle = cpu.execute(opcode);
        for i in 0..8u8 {
            if i == 6 { continue; }
            let expect = if i == ddd { REG_INIT[sss as usize] }
                             else { REG_INIT[i as usize] };
            assert_eq!(reg_get(&cpu, i), expect, "0x{:02X}: sss -> ddd (index: {:02X})", opcode, i);
        }
        assert_eq!(cpu.registers.F, f_before, "0x{:02X}: F 不変", opcode);
        assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
        count += 1;
    }
    assert_eq!(count, 49, "count -> 49");
}

#[ignore]
#[test]
fn ld_r_hl_reads_memory() {
    todo!()
}

#[ignore]
#[test]
fn ld_r_hl_all_registers() {
    todo!()
}

#[ignore]
#[test]
fn ld_hl_r_writes_memory() {
    todo!()
}

#[ignore]
#[test]
fn ld_hl_r_all_registers() {
    todo!()
}

#[ignore]
#[test]
fn ld_hl_n_writes_immediate() {
    todo!()
}

#[ignore]
#[test]
fn ld_bc_de_indirect_both_directions() {
    todo!()
}

#[test]
fn ld_hl_inc_updates_hl() {
    // === 0x22 (LD (HL+), A) ===
    let mut cpu = new_cpu();
    let opcode = 0x22; // LD (HL+), A
    let addr = 0xC100u16;
    let a_before = 0x5Au8;
    let mem_next = 0xA5u8;
    cpu.registers.A = a_before;
    cpu.registers.set_hl(addr);
    cpu.bus.write(addr + 1, mem_next);
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.bus.read(addr), a_before, "0x{:02X}: A -> (HL)", opcode);
    assert_eq!(cpu.registers.get_hl(), addr + 1, "0x{:02X}: HL -> HL+1", opcode);
    assert_eq!(cpu.bus.read(addr + 1), mem_next, "0x{:02X}: (HL+1) 不変", opcode);
    assert_eq!(cpu.registers.A, a_before, "0x{:02X}: A 不変", opcode);
    assert_eq!(cpu.registers.F, F_BEFORE, "0x{:02X}: F 不変", opcode);
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 8);

    // === 0x2A (LD A, (HL+)) ===
    let mut cpu = new_cpu();
    let opcode = 0x2A; // LD A, (HL+)
    let addr = 0xC200u16;
    let mem = 0x5Au8;
    let mem_next = 0xA5u8;
    let a_before = 0x3Cu8;
    cpu.registers.A = a_before;
    cpu.registers.set_hl(addr);
    cpu.bus.write(addr, mem);
    cpu.bus.write(addr + 1, mem_next);
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, mem, "0x{:02X}: (HL) -> A", opcode);
    assert_eq!(cpu.registers.get_hl(), addr + 1, "0x{:02X}: HL -> HL+1", opcode);
    assert_eq!(cpu.bus.read(addr), mem, "0x{:02X}: (HL) 不変", opcode);
    assert_eq!(cpu.bus.read(addr + 1), mem_next, "0x{:02X}: (HL+1) 不変", opcode);
    assert_eq!(cpu.registers.F, F_BEFORE, "0x{:02X}: F 不変", opcode);
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 8);
}

#[test]
fn ld_hl_dec_updates_hl() {
    // === 0x32 (LD (HL-), A) ===
    let mut cpu = new_cpu();
    let opcode = 0x32; // LD (HL-), A
    let addr = 0xC100u16;
    let a_before = 0x5Au8;
    let mem_prev = 0xA5u8;
    cpu.registers.A = a_before;
    cpu.registers.set_hl(addr);
    cpu.bus.write(addr - 1, mem_prev);
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.bus.read(addr), a_before, "0x{:02X}: A -> (HL)", opcode);
    assert_eq!(cpu.registers.get_hl(), addr - 1, "0x{:02X}: HL -> HL-1", opcode);
    assert_eq!(cpu.bus.read(addr - 1), mem_prev, "0x{:02X}: (HL-1) 不変", opcode);
    assert_eq!(cpu.registers.A, a_before, "0x{:02X}: A 不変", opcode);
    assert_eq!(cpu.registers.F, F_BEFORE, "0x{:02X}: F 不変", opcode);
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 8);

    // === 0x3A (LD A, (HL-)) ===
    let mut cpu = new_cpu();
    let opcode = 0x3A; // LD A, (HL-)
    let addr = 0xC200u16;
    let mem = 0x5Au8;
    let mem_prev = 0xA5u8;
    let a_before = 0x3Cu8;
    cpu.registers.A = a_before;
    cpu.registers.set_hl(addr);
    cpu.bus.write(addr, mem);
    cpu.bus.write(addr - 1, mem_prev);
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, mem, "0x{:02X}: (HL) -> A", opcode);
    assert_eq!(cpu.registers.get_hl(), addr - 1, "0x{:02X}: HL -> HL-1", opcode);
    assert_eq!(cpu.bus.read(addr), mem, "0x{:02X}: (HL) 不変", opcode);
    assert_eq!(cpu.bus.read(addr - 1), mem_prev, "0x{:02X}: (HL-1) 不変", opcode);
    assert_eq!(cpu.registers.F, F_BEFORE, "0x{:02X}: F 不変", opcode);
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 8);
}

#[ignore]
#[test]
fn ld_a_nn_absolute_both_directions() {
    todo!()
}

#[ignore]
#[test]
fn ldh_n_a_uses_ff00_offset() {
    todo!()
}

#[ignore]
#[test]
fn ldh_c_a_uses_ff00_offset() {
    todo!()
}

// ==================== 16bitロード, スタック ====================
#[ignore]
#[test]
fn ld_rr_nn_little_endian() {
    todo!()
}

#[ignore]
#[test]
fn ld_rr_nn_all_pairs() {
    todo!()
}

#[ignore]
#[test]
fn ld_nn_sp_writes_low_byte_first() {
    todo!()
}

#[ignore]
#[test]
fn ld_sp_hl_copies() {
    todo!()
}

#[ignore]
#[test]
fn ld_hl_sp_e_flags_from_low_byte() {
    todo!()
}

#[ignore]
#[test]
fn ld_hl_sp_e_negative_offset() {
    todo!()
}

#[ignore]
#[test]
fn push_bc_stores_high_byte_at_higher_address() {
    todo!()
}

#[ignore]
#[test]
fn pop_bc_loads_low_byte_first() {
    todo!()
}

#[ignore]
#[test]
fn push_pop_all_pairs() {
    todo!()
}

#[test]
fn pop_af_masks_lower_nibble() {
    let mut cpu = new_cpu();
    let opcode = 0xF1; // POP AF
    let sp_before = 0xCF00u16;
    let mem = 0x5Au8;
    let mem_next = 0xA5u8;
    let a_before = 0x3Cu8;
    cpu.registers.A = a_before;
    cpu.registers.SP = sp_before;
    cpu.bus.write(sp_before, mem);
    cpu.bus.write(sp_before + 1, mem_next);
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x50, "(SP) -> F 下位ニブルは0"); // mem&0xF0 = 0x50
    assert_eq!(cpu.registers.A, mem_next, "(SP+1) -> A");
    assert_eq!(cpu.registers.SP, sp_before + 2, "SP -> SP+2");
    assert_eq!(cpu.bus.read(sp_before), mem, "(SP) 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 12);
}

// ==================== 8bit算術論理 ====================
#[test]
fn add_a_b_sum_and_zero() {
    // === 繰り上がりなし ===
    let mut cpu = new_cpu();
    let opcode = 0x80u8; // ADD A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a+b = 0x96
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x96, "A + B -> A");
    assert_eq!(cpu.registers.F, 0x20, "ZNHC -> 0010");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === 繰り上がりあり ===
    let mut cpu = new_cpu();
    let b_before = 0xA6u8; // a+b = 0x00
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x00, "A + B -> A");
    assert_eq!(cpu.registers.F, 0xB0, "ZNHC -> 1011");
}

#[test]
fn add_a_b_half_carry() {
    // === Hが立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x80u8; // ADD A, B
    let a_before = 0x3Cu8;
    let b_before = 0xA5u8; // a+b = 0xE1
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x20, "ZNHC -> 0010");
    assert_eq!(cpu.registers.A, 0xE1, "A + B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === Hが立たない ===
    let mut cpu = new_cpu();
    let a_before = 0x5Au8;
    cpu.registers.A = a_before;
    cpu.registers.B = b_before; // a+b = 0xFF
    cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x00, "ZNHC -> 0000");
    assert_eq!(cpu.registers.A, 0xFF, "A + B -> A");
}

#[test]
fn add_a_b_carry() {
    // === Cが立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x80u8; // ADD A, B
    let a_before = 0xA5u8;
    let b_before = 0xC3u8; // a+b = 0x68
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x68, "A + B -> A");
    assert_eq!(cpu.registers.F, 0x10, "ZNHC -> 0001");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === Cが立たない ===
    let mut cpu = new_cpu();
    let a_before = 0x3Cu8;
    cpu.registers.A = a_before;
    cpu.registers.B = b_before; // a+b = 0xFF
    cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0xFF, "A + B -> A");
    assert_eq!(cpu.registers.F, 0x00, "ZNHC -> 0000");
}

#[test]
fn adc_a_b_adds_carry_in() {
    let mut cpu = new_cpu();
    let opcode = 0x88u8; // ADC A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a+b+c = 0x96+1
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x97, "A + B + C -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0x20, "ZNHC -> 0010");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[test]
fn adc_a_b_half_carry_includes_carry_in() {
    let mut cpu = new_cpu();
    let opcode = 0x88u8; // ADC A, B
    let a_before = 0x5Au8;
    let b_before = 0xA5u8; // a+b+c = 0x00
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x00, "A + B + C -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0xB0, "ZNHC -> 1011");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[test]
fn adc_a_b_without_carry_matches_add() {
    let mut cpu = new_cpu();
    let opcode = 0x80u8; // ADD A, B
    let a_before = 0x5Au8;
    let b_before = 0xA5u8; // a+b = 0xFF
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.execute(opcode); // まずはADDの結果を保存(比較のため)
    let a_after_add = cpu.registers.A;
    let f_after_add = cpu.registers.F;

    let mut cpu = new_cpu();
    let opcode = 0x88u8; // ADC A, B
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::C, false); // C=0の挙動確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, a_after_add, "C=0においてADDと同等");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, f_after_add, "ZNHC ADDと同等");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[test]
fn sub_a_b_difference_and_zero() {
    // === Z立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x90u8; // SUB A, B
    let a_before = 0x5Au8;
    let b_before = 0x5Au8; // a-b = 0x00
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::Z, false); // 命令によってZが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x00, "A - B -> A");
    assert_eq!(cpu.registers.F, 0xC0, "ZNHC -> 1100");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === Z立たない ===
    let mut cpu = new_cpu();
    let b_before = 0x3Cu8; // a-b = 0x1E
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x1E, "A - B -> A");
    assert_eq!(cpu.registers.F, 0x60, "ZNHC -> 0110");
}

#[test]
fn sub_a_b_sets_n_flag() {
    let mut cpu = new_cpu();
    let opcode = 0x90u8; // SUB A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a-b = 0x1E
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::N, false); // 命令によってNが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x1E, "A - B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0x60, "ZNHC -> 0110"); // SUBなのでN=1
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[test]
fn sub_a_b_half_borrow() {
    // === H立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x90u8; // SUB A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a-b = 0x1E
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::H, false); // 命令によってHが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x1E, "A - B -> A");
    assert_eq!(cpu.registers.F, 0x60, "ZNHC -> 0110"); // A下位 < B下位なのでH=1
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === H立たない ===
    let mut cpu = new_cpu();
    let a_before = 0x3Cu8;
    let b_before = 0xA5u8; // a-b = 0x97
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x97, "A - B -> A");
    assert_eq!(cpu.registers.F, 0x50, "ZNHC -> 0101");
}

#[test]
fn sub_a_b_borrow() {
    // === C立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x90u8; // SUB A, B
    let a_before = 0xA5u8;
    let b_before = 0xC3u8; // a-b = 0xE2
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::C, false); // 命令によってCが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0xE2, "A - B -> A");
    assert_eq!(cpu.registers.F, 0x50, "ZNHC -> 0101"); // A < BなのでC=1
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === C立たない ===
    let mut cpu = new_cpu();
    let a_before = 0xC3u8;
    let b_before = 0xA5u8; // a-b = 0x1E
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x1E, "A - B -> A");
    assert_eq!(cpu.registers.F, 0x60, "ZNHC -> 0110");
}

#[ignore]
#[test]
fn sbc_a_b_subtracts_carry_in() {
    todo!()
}

#[ignore]
#[test]
fn sbc_a_b_half_borrow_includes_carry_in() {
    todo!()
}

#[ignore]
#[test]
fn and_a_b_result_and_zero() {
    todo!()
}

#[test]
fn and_a_b_sets_half_carry_clears_carry() {
    let mut cpu = new_cpu();
    let opcode = 0xA0u8; // AND A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a&b = 0x18
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::H, false); // 命令によってHが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x18, "A & B -> A");
    assert_eq!(cpu.registers.F, 0x20, "ZNHC -> 0010"); // 入力問わずH=1, C=0
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[ignore]
#[test]
fn or_a_b_result_and_zero() {
    todo!()
}

#[ignore]
#[test]
fn or_a_b_clears_n_h_c() {
    todo!()
}

#[ignore]
#[test]
fn xor_a_b_result_and_zero() {
    todo!()
}

#[ignore]
#[test]
fn xor_a_b_clears_n_h_c() {
    todo!()
}

#[test]
fn cp_a_b_does_not_modify_a() {
    let mut cpu = new_cpu();
    let opcode = 0xB8u8; // CP A, B
    let a_before = 0x5Au8;
    let b_before = 0x3Cu8; // a-b = 0x1E
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, a_before, "A 不変"); // subと同じフラグ変化だがAに書き込まない
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0x60, "ZNHC -> 0110"); // 入力問わずH=1, C=0
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[ignore]
#[test]
fn cp_a_b_equal_sets_zero() {
    todo!()
}

#[ignore]
#[test]
fn cp_a_b_flags_match_sub() {
    todo!()
}

#[ignore]
#[test]
fn add_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn adc_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn sub_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn sbc_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn and_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn or_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn xor_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn cp_a_all_sources() {
    todo!()
}

#[ignore]
#[test]
fn alu_a_a_self_operand() {
    todo!()
}

// ==================== INC/DEC ====================
#[test]
fn inc_r_preserves_carry() {
    let mut cpu = new_cpu();
    let opcode = 0x04u8; // INC B
    let b_before = 0x5Au8;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x10, "ZNHC -> 0001"); // 入力問わずCに触れない(C=1)
    assert_eq!(cpu.registers.B, 0x5B, "B + 1 -> B");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    let mut cpu = new_cpu();
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::C, false); // 命令によってCが変わらないことを確認
    cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x00, "ZNHC -> 0000"); // 入力問わずCに触れない(C=0)
}

#[test]
fn inc_r_half_carry_at_0f() {
    // === Hが立つ ===
    let mut cpu = new_cpu();
    let opcode = 0x04u8; // INC B
    cpu.registers.B = 0x5Fu8; // 下位4bitがFなら立つ
    cpu.registers.set_flag(Flag::H, false); // 命令によってHが立つことを確認
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.B, 0x60, "B + 1 -> B");
    assert_eq!(cpu.registers.F, 0x30, "ZNHC -> 0011");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === Hが立たない ===
    let mut cpu = new_cpu();
    cpu.registers.B = 0x5Eu8; // 下位4bitがFでないので立たない
    cpu.execute(opcode);

    assert_eq!(cpu.registers.B, 0x5F, "B + 1 -> B");
    assert_eq!(cpu.registers.F, 0x10, "ZNHC -> 0001");
}

#[ignore]
#[test]
fn inc_r_zero_at_ff() {
    todo!()
}

#[ignore]
#[test]
fn inc_r_clears_n_flag() {
    todo!()
}

#[ignore]
#[test]
fn inc_r_all_registers() {
    todo!()
}

#[test]
fn dec_r_preserves_carry() {
    let mut cpu = new_cpu();
    let opcode = 0x05u8; // DEC B
    let b_before = 0x5Au8;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x50, "ZNHC -> 0101"); // 入力問わずCに触れない(C=1)
    assert_eq!(cpu.registers.B, 0x59, "B - 1 -> B");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    let mut cpu = new_cpu();
    cpu.registers.B = b_before;
    cpu.registers.set_flag(Flag::C, false); // 命令によってCが変わらないことを確認
    cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x40, "ZNHC -> 0100"); // 入力問わずCに触れない(C=0)
}

#[ignore]
#[test]
fn dec_r_half_borrow_at_00() {
    todo!()
}

#[ignore]
#[test]
fn dec_r_zero_at_01() {
    todo!()
}

#[ignore]
#[test]
fn dec_r_sets_n_flag() {
    todo!()
}

#[ignore]
#[test]
fn dec_r_all_registers() {
    todo!()
}

#[ignore]
#[test]
fn inc_dec_hl_indirect_updates_memory() {
    todo!()
}

// ==================== 16bit算出命令 ====================
#[ignore = "次にやる"]
#[test]
fn add_hl_rr_half_carry_at_bit11() {
    todo!()
}

#[ignore]
#[test]
fn add_hl_rr_carry_at_bit15() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn add_hl_rr_preserves_zero_flag() {
    todo!()
}

#[ignore]
#[test]
fn add_hl_rr_clears_n_flag() {
    todo!()
}

#[ignore]
#[test]
fn add_hl_rr_all_pairs() {
    todo!()
}

#[ignore]
#[test]
fn add_hl_hl_doubles() {
    todo!()
}

#[ignore]
#[test]
fn inc_rr_does_not_affect_flags() {
    todo!()
}

#[ignore]
#[test]
fn dec_rr_does_not_affect_flags() {
    todo!()
}

#[ignore]
#[test]
fn add_sp_e_flags_from_low_byte() {
    todo!()
}

#[ignore]
#[test]
fn add_sp_e_negative_offset() {
    todo!()
}

// ==================== ローテート, フラグ操作 ====================
#[ignore = "次にやる"]
#[test]
fn rlca_rotates_left_through_bit7() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn rrca_rotates_right_through_bit0() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn rla_shifts_carry_in_to_bit0() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn rra_shifts_carry_in_to_bit7() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn rotate_a_always_clears_zero() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn daa_after_addition() {
    todo!()
}

#[ignore]
#[test]
fn daa_after_subtraction() {
    todo!()
}

#[ignore]
#[test]
fn daa_sets_carry_on_upper_adjust() {
    todo!()
}

#[ignore]
#[test]
fn cpl_inverts_a_and_sets_n_h() {
    todo!()
}

#[ignore]
#[test]
fn scf_sets_carry_preserving_zero() {
    todo!()
}

#[ignore]
#[test]
fn ccf_toggles_carry_preserving_zero() {
    todo!()
}

// ==================== ジャンプ ====================
#[ignore]
#[test]
fn jp_nn_sets_pc() {
    todo!()
}

#[ignore]
#[test]
fn jp_cc_nn_taken() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn jp_cc_nn_not_taken_consumes_operand() {
    todo!()
}

#[ignore]
#[test]
fn jp_cc_nn_all_conditions() {
    todo!()
}

#[ignore]
#[test]
fn jp_hl_uses_hl_directly() {
    todo!()
}

#[ignore]
#[test]
fn jr_e_forward() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn jr_e_backward() {
    todo!()
}

#[ignore]
#[test]
fn jr_e_is_relative_to_next_instruction() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn jr_cc_e_not_taken_consumes_operand() {
    todo!()
}

#[ignore]
#[test]
fn jr_cc_e_all_conditions() {
    todo!()
}

// ==================== CALL/RET/RST ====================
#[ignore]
#[test]
fn call_nn_pushes_return_address() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn call_nn_return_address_points_after_operand() {
    todo!()
}

#[ignore]
#[test]
fn call_cc_not_taken_consumes_operand() {
    todo!()
}

#[ignore]
#[test]
fn call_cc_all_conditions() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn ret_pops_pc() {
    todo!()
}

#[ignore]
#[test]
fn ret_cc_taken_and_not_taken() {
    todo!()
}

#[ignore]
#[test]
fn reti_enables_ime() {
    todo!()
}

#[ignore]
#[test]
fn rst_pushes_and_jumps_to_vector() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn rst_all_eight_vectors() {
    todo!()
}

// ==================== 横断 ====================
#[ignore = "次にやる"]
#[test]
fn flag_lower_nibble_always_zero() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn cycles_are_multiples_of_four() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn defined_opcodes_do_not_panic() {
    todo!()
}

#[ignore = "次にやる"]
#[test]
fn pc_never_moves_backward_on_non_jump() {
    todo!()
}
