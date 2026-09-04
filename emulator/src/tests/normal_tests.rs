use crate::cpu::{Cpu, registers::Flag};
use crate::bus::Bus;

// ==================== 基本制御系 ====================
#[ignore]
#[test]
fn nop_costs_4_cycles() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn stop_consumes_one_operand_byte() {
    let mut cpu = Cpu::new(Bus::new());
    let start = 0xC100u16;
    cpu.registers.PC = start; // wram
    let cycle = cpu.execute(0x10); // STOP
    assert_eq!(cpu.registers.PC, start+1, "STOP -> PC+1");
    assert_eq!(cycle, 4, "cycle -> 4");
}

#[ignore]
#[test]
fn halt_sets_halted_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn di_clears_ime() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ei_sets_pending_not_ime() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
#[should_panic(expected = "未定義のオペコードです")]
fn undefined_opcode_panics() {
    // 未定義は次の通り。代表値のみをテストする。パニックのテストなのでAssertは書かない
    // 0xD3  0xDB  0xDD  0xE3  0xE4  0xEB  0xEC  0xED  0xF4  0xFC  0xFD
    let mut cpu = Cpu::new(Bus::new());
    cpu.execute(0xDD);
}

// ==================== 8bitロード ====================
#[test]
fn ld_r_n_loads_immediate() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_r_n_all_registers() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_r_r_copies_source() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn ld_r_r_all_opcodes() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_r_hl_reads_memory() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_r_hl_all_registers() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_hl_r_writes_memory() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_hl_r_all_registers() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_hl_n_writes_immediate() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_bc_de_indirect_both_directions() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn ld_hl_inc_dec_updates_hl() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_a_nn_absolute_both_directions() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ldh_n_a_uses_ff00_offset() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ldh_c_a_uses_ff00_offset() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== 16bitロード, スタック ====================
#[ignore]
#[test]
fn ld_rr_nn_little_endian() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_rr_nn_all_pairs() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_nn_sp_writes_low_byte_first() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_sp_hl_copies() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_hl_sp_e_flags_from_low_byte() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ld_hl_sp_e_negative_offset() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn push_bc_stores_high_byte_at_higher_address() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn pop_bc_loads_low_byte_first() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn push_pop_all_pairs() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn pop_af_masks_lower_nibble() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== 8bit算術論理 ====================
#[test]
fn add_a_b_sum_and_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn add_a_b_half_carry() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn add_a_b_carry() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn adc_a_b_adds_carry_in() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn adc_a_b_half_carry_includes_carry_in() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn adc_a_b_without_carry_matches_add() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn sub_a_b_difference_and_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn sub_a_b_sets_n_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn sub_a_b_half_borrow() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn sub_a_b_borrow() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn sbc_a_b_subtracts_carry_in() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn sbc_a_b_half_borrow_includes_carry_in() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn and_a_b_result_and_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn and_a_b_sets_half_carry_clears_carry() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn or_a_b_result_and_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn or_a_b_clears_n_h_c() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn xor_a_b_result_and_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn xor_a_b_clears_n_h_c() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn cp_a_b_does_not_modify_a() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn cp_a_b_equal_sets_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn cp_a_b_flags_match_sub() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn adc_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn sub_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn sbc_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn and_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn or_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn xor_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn cp_a_all_sources() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn alu_a_a_self_operand() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== INC/DEC ====================
#[test]
fn inc_r_preserves_carry() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn inc_r_half_carry_at_0f() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn inc_r_zero_at_ff() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn inc_r_clears_n_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn inc_r_all_registers() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn dec_r_preserves_carry() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn dec_r_half_borrow_at_00() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn dec_r_zero_at_01() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn dec_r_sets_n_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn dec_r_all_registers() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn inc_dec_hl_indirect_updates_memory() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== 16bit算出命令 ====================
#[test]
fn add_hl_rr_half_carry_at_bit11() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_hl_rr_carry_at_bit15() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn add_hl_rr_preserves_zero_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_hl_rr_clears_n_flag() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_hl_rr_all_pairs() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_hl_hl_doubles() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn inc_rr_does_not_affect_flags() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn dec_rr_does_not_affect_flags() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_sp_e_flags_from_low_byte() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn add_sp_e_negative_offset() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== ローテート, フラグ操作 ====================
#[test]
fn rlca_rotates_left_through_bit7() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn rrca_rotates_right_through_bit0() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn rla_shifts_carry_in_to_bit0() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn rra_shifts_carry_in_to_bit7() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn rotate_a_always_clears_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn daa_after_addition() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn daa_after_subtraction() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn daa_sets_carry_on_upper_adjust() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn cpl_inverts_a_and_sets_n_h() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn scf_sets_carry_preserving_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ccf_toggles_carry_preserving_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== ジャンプ ====================
#[ignore]
#[test]
fn jp_nn_sets_pc() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jp_cc_nn_taken() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn jp_cc_nn_not_taken_consumes_operand() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jp_cc_nn_all_conditions() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jp_hl_uses_hl_directly() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jr_e_forward() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn jr_e_backward() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jr_e_is_relative_to_next_instruction() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn jr_cc_e_not_taken_consumes_operand() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn jr_cc_e_all_conditions() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== CALL/RET/RST ====================
#[ignore]
#[test]
fn call_nn_pushes_return_address() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn call_nn_return_address_points_after_operand() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn call_cc_not_taken_consumes_operand() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn call_cc_all_conditions() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn ret_pops_pc() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn ret_cc_taken_and_not_taken() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn reti_enables_ime() {
    let mut cpu = Cpu::new(Bus::new());
}

#[ignore]
#[test]
fn rst_pushes_and_jumps_to_vector() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn rst_all_eight_vectors() {
    let mut cpu = Cpu::new(Bus::new());
}

// ==================== 横断 ====================
#[test]
fn flag_lower_nibble_always_zero() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn cycles_are_multiples_of_four() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn defined_opcodes_do_not_panic() {
    let mut cpu = Cpu::new(Bus::new());
}

#[test]
fn pc_never_moves_backward_on_non_jump() {
    let mut cpu = Cpu::new(Bus::new());
}
