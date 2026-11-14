use crate::cpu::{Cpu, registers::Flag};
use crate::bus::Bus;
use super::common::*;

// ==================== シフト/ローテート（0x00–0x3F）====================
#[ignore]
#[test]
fn rlc_r_rotates_left_through_bit7() {
    todo!()
}

#[ignore]
#[test]
fn rrc_r_rotates_right_through_bit0() {
    todo!()
}

#[ignore]
#[test]
fn rl_r_shifts_carry_in_to_bit0() {
    todo!()
}

#[ignore]
#[test]
fn rr_r_shifts_carry_in_to_bit7() {
    todo!()
}

#[ignore]
#[test]
fn sla_r_clears_bit0() {
    todo!()
}

#[ignore]
#[test]
fn sra_r_keeps_bit7() {
    todo!()
}

#[ignore]
#[test]
fn srl_r_clears_bit7() {
    todo!()
}

#[ignore]
#[test]
fn swap_r_exchanges_nibbles() {
    todo!()
}

#[ignore]
#[test]
fn swap_r_clears_carry() {
    todo!()
}

#[ignore]
#[test]
fn rlc_a_sets_zero_unlike_rlca() {
    todo!()
}

#[ignore]
#[test]
fn shift_rotate_hl_writes_memory() {
    todo!()
}

#[ignore]
#[test]
fn shift_rotate_all_opcodes() {
    todo!()
}

// ==================== BIT（0x40–0x7F）====================
#[ignore]
#[test]
fn bit_n_r_sets_zero_when_bit_clear() {
    todo!()
}

#[ignore]
#[test]
fn bit_n_r_clears_zero_when_bit_set() {
    todo!()
}

#[ignore]
#[test]
fn bit_n_r_preserves_carry() {
    todo!()
}

#[ignore]
#[test]
fn bit_n_r_does_not_modify_target() {
    todo!()
}

#[ignore]
#[test]
fn bit_n_hl_costs_12_cycles() {
    todo!()
}

#[ignore]
#[test]
fn bit_n_all_opcodes() {
    todo!()
}

// ==================== RES/SET（0x80–0xFF）====================
#[ignore]
#[test]
fn bit_n_all_opcores_n_r_clears_only_bit_ndes() {
    todo!()
}

#[ignore]
#[test]
fn set_n_r_sets_only_bit_n() {
    todo!()
}

#[ignore]
#[test]
fn res_set_do_not_affect_flags() {
    todo!()
}

#[ignore]
#[test]
fn res_set_hl_writes_memory() {
    todo!()
}

#[ignore]
#[test]
fn res_n_all_opcodes() {
    todo!()
}

#[ignore]
#[test]
fn set_n_all_opcodes() {
    todo!()
}

// ==================== 横断 ====================
#[ignore]
#[test]
fn cb_all_opcodes_cycles_and_pc() {
    todo!()
}
