use std::iter::Cycle;

use crate::cpu::{Cpu, registers::Flag};
use crate::bus::Bus;

// ==================== テストで利用する定数/関数 ====================
const PC_BEFORE: u16 = 0xC800;   // 命令列の位置（データ用アドレスと区別）
const F_BEFORE:  u8  = 0xF0;     // 下位4bitは実機で常に0
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
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn di_clears_ime() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ei_sets_pending_not_ime() {
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_r_r_copies_source() {
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_r_hl_all_registers() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_hl_r_writes_memory() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_hl_r_all_registers() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_hl_n_writes_immediate() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_bc_de_indirect_both_directions() {
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ldh_n_a_uses_ff00_offset() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ldh_c_a_uses_ff00_offset() {
    let mut cpu = new_cpu();
}

// ==================== 16bitロード, スタック ====================
#[ignore]
#[test]
fn ld_rr_nn_little_endian() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_rr_nn_all_pairs() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_nn_sp_writes_low_byte_first() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_sp_hl_copies() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_hl_sp_e_flags_from_low_byte() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ld_hl_sp_e_negative_offset() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn push_bc_stores_high_byte_at_higher_address() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn pop_bc_loads_low_byte_first() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn push_pop_all_pairs() {
    let mut cpu = new_cpu();
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
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0x20, "ZNHC -> 0010");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === 繰り上がりあり ===
    let mut cpu = new_cpu();
    let b_before = 0xA6u8; // a+b = 0x00
    cpu.registers.A = a_before;
    cpu.registers.B = b_before;
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.A, 0x00, "A + B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_eq!(cpu.registers.F, 0xB0, "ZNHC -> 1011");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
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
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x00, "ZNHC -> 0000");
    assert_eq!(cpu.registers.A, 0xFF, "A + B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
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

    assert_eq!(cpu.registers.F, 0x10, "ZNHC -> 0001");
    assert_eq!(cpu.registers.A, 0x68, "A + B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);

    // === Cが立たない ===
    let mut cpu = new_cpu();
    let a_before = 0x3Cu8;
    cpu.registers.A = a_before;
    cpu.registers.B = b_before; // a+b = 0xFF
    let cycle = cpu.execute(opcode);

    assert_eq!(cpu.registers.F, 0x00, "ZNHC -> 0000");
    assert_eq!(cpu.registers.A, 0xFF, "A + B -> A");
    assert_eq!(cpu.registers.B, b_before, "B 不変");
    assert_pc_and_cycle(&cpu, opcode, PC_BEFORE, cycle, 4);
}

#[test]
fn adc_a_b_adds_carry_in() {
    let mut cpu = new_cpu();
}

#[test]
fn adc_a_b_half_carry_includes_carry_in() {
    let mut cpu = new_cpu();
}

#[test]
fn adc_a_b_without_carry_matches_add() {
    let mut cpu = new_cpu();
}

#[test]
fn sub_a_b_difference_and_zero() {
    let mut cpu = new_cpu();
}

#[test]
fn sub_a_b_sets_n_flag() {
    let mut cpu = new_cpu();
}

#[test]
fn sub_a_b_half_borrow() {
    let mut cpu = new_cpu();
}

#[test]
fn sub_a_b_borrow() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn sbc_a_b_subtracts_carry_in() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn sbc_a_b_half_borrow_includes_carry_in() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn and_a_b_result_and_zero() {
    let mut cpu = new_cpu();
}

#[test]
fn and_a_b_sets_half_carry_clears_carry() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn or_a_b_result_and_zero() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn or_a_b_clears_n_h_c() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn xor_a_b_result_and_zero() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn xor_a_b_clears_n_h_c() {
    let mut cpu = new_cpu();
}

#[test]
fn cp_a_b_does_not_modify_a() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn cp_a_b_equal_sets_zero() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn cp_a_b_flags_match_sub() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn adc_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn sub_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn sbc_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn and_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn or_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn xor_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn cp_a_all_sources() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn alu_a_a_self_operand() {
    let mut cpu = new_cpu();
}

// ==================== INC/DEC ====================
#[test]
fn inc_r_preserves_carry() {
    let mut cpu = new_cpu();
}

#[test]
fn inc_r_half_carry_at_0f() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn inc_r_zero_at_ff() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn inc_r_clears_n_flag() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn inc_r_all_registers() {
    let mut cpu = new_cpu();
}

#[test]
fn dec_r_preserves_carry() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn dec_r_half_borrow_at_00() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn dec_r_zero_at_01() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn dec_r_sets_n_flag() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn dec_r_all_registers() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn inc_dec_hl_indirect_updates_memory() {
    let mut cpu = new_cpu();
}

// ==================== 16bit算出命令 ====================
#[test]
fn add_hl_rr_half_carry_at_bit11() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_hl_rr_carry_at_bit15() {
    let mut cpu = new_cpu();
}

#[test]
fn add_hl_rr_preserves_zero_flag() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_hl_rr_clears_n_flag() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_hl_rr_all_pairs() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_hl_hl_doubles() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn inc_rr_does_not_affect_flags() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn dec_rr_does_not_affect_flags() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_sp_e_flags_from_low_byte() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn add_sp_e_negative_offset() {
    let mut cpu = new_cpu();
}

// ==================== ローテート, フラグ操作 ====================
#[test]
fn rlca_rotates_left_through_bit7() {
    let mut cpu = new_cpu();
}

#[test]
fn rrca_rotates_right_through_bit0() {
    let mut cpu = new_cpu();
}

#[test]
fn rla_shifts_carry_in_to_bit0() {
    let mut cpu = new_cpu();
}

#[test]
fn rra_shifts_carry_in_to_bit7() {
    let mut cpu = new_cpu();
}

#[test]
fn rotate_a_always_clears_zero() {
    let mut cpu = new_cpu();
}

#[test]
fn daa_after_addition() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn daa_after_subtraction() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn daa_sets_carry_on_upper_adjust() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn cpl_inverts_a_and_sets_n_h() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn scf_sets_carry_preserving_zero() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ccf_toggles_carry_preserving_zero() {
    let mut cpu = new_cpu();
}

// ==================== ジャンプ ====================
#[ignore]
#[test]
fn jp_nn_sets_pc() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jp_cc_nn_taken() {
    let mut cpu = new_cpu();
}

#[test]
fn jp_cc_nn_not_taken_consumes_operand() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jp_cc_nn_all_conditions() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jp_hl_uses_hl_directly() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jr_e_forward() {
    let mut cpu = new_cpu();
}

#[test]
fn jr_e_backward() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jr_e_is_relative_to_next_instruction() {
    let mut cpu = new_cpu();
}

#[test]
fn jr_cc_e_not_taken_consumes_operand() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn jr_cc_e_all_conditions() {
    let mut cpu = new_cpu();
}

// ==================== CALL/RET/RST ====================
#[ignore]
#[test]
fn call_nn_pushes_return_address() {
    let mut cpu = new_cpu();
}

#[test]
fn call_nn_return_address_points_after_operand() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn call_cc_not_taken_consumes_operand() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn call_cc_all_conditions() {
    let mut cpu = new_cpu();
}

#[test]
fn ret_pops_pc() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn ret_cc_taken_and_not_taken() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn reti_enables_ime() {
    let mut cpu = new_cpu();
}

#[ignore]
#[test]
fn rst_pushes_and_jumps_to_vector() {
    let mut cpu = new_cpu();
}

#[test]
fn rst_all_eight_vectors() {
    let mut cpu = new_cpu();
}

// ==================== 横断 ====================
#[test]
fn flag_lower_nibble_always_zero() {
    let mut cpu = new_cpu();
}

#[test]
fn cycles_are_multiples_of_four() {
    let mut cpu = new_cpu();
}

#[test]
fn defined_opcodes_do_not_panic() {
    let mut cpu = new_cpu();
}

#[test]
fn pc_never_moves_backward_on_non_jump() {
    let mut cpu = new_cpu();
}
