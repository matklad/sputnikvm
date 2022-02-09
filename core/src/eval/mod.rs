#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod misc;

use crate::{ExitError, ExitReason, ExitSucceed, Machine, Opcode};
use core::ops::{BitAnd, BitOr, BitXor};
use primitive_types::{H256, U256};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Control {
	Continue(usize),
	Exit(ExitReason),
	Jump(usize),
	Trap(Opcode),
}

extern "C" {
	fn gas_profile_start();
	fn gas_profile_stop(cost: u32);
}

#[cfg(not(target_arch = "wasm32"))]
mod stub {
	#[no_mangle]
	extern "C" fn gas_profile_start() {}
	#[no_mangle]
	extern "C" fn gas_profile_stop(cost: u32) {}
}

fn eval_stop(_state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = Control::Exit(ExitSucceed::Stopped.into());
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_add(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_tuple!(state, overflowing_add);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mul(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_tuple!(state, overflowing_mul);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_sub(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_tuple!(state, overflowing_sub);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_div(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::div);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_sdiv(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::sdiv);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mod(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::rem);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_smod(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::srem);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_addmod(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op3_u256_fn!(state, self::arithmetic::addmod);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mulmod(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op3_u256_fn!(state, self::arithmetic::mulmod);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_exp(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::exp);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_signextend(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::arithmetic::signextend);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_lt(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_bool_ref!(state, lt);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_gt(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_bool_ref!(state, gt);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_slt(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::slt);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_sgt(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::sgt);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_eq(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_bool_ref!(state, eq);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_iszero(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op1_u256_fn!(state, self::bitwise::iszero);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_and(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256!(state, bitand);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_or(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256!(state, bitor);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_xor(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256!(state, bitxor);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_not(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op1_u256_fn!(state, self::bitwise::not);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_byte(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::byte);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_shl(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::shl);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_shr(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::shr);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_sar(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = op2_u256_fn!(state, self::bitwise::sar);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_codesize(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::codesize(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_codecopy(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::codecopy(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_calldataload(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::calldataload(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_calldatasize(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::calldatasize(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_calldatacopy(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::calldatacopy(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_pop(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::pop(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mload(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::mload(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mstore(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::mstore(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_mstore8(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::mstore8(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_jump(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::jump(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_jumpi(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::jumpi(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_pc(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::pc(state, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_msize(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::msize(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_jumpdest(_state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = Control::Continue(1);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push1(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 1, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push2(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 2, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push3(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 3, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push4(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 4, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push5(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 5, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push6(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 6, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push7(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 7, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push8(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 8, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push9(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 9, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push10(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 10, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push11(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 11, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push12(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 12, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push13(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 13, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push14(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 14, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push15(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 15, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push16(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 16, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push17(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 17, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push18(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 18, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push19(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 19, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push20(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 20, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push21(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 21, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push22(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 22, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push23(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 23, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push24(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 24, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push25(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 25, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push26(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 26, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push27(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 27, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push28(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 28, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push29(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 29, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push30(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 30, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push31(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 31, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_push32(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::push(state, 32, position);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup1(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 1);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup2(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 2);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup3(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 3);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup4(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 4);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup5(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 5);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup6(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 6);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup7(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 7);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup8(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 8);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup9(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 9);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup10(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 10);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup11(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 11);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup12(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 12);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup13(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 13);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup14(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 14);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup15(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 15);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_dup16(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::dup(state, 16);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap1(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 1);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap2(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 2);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap3(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 3);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap4(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 4);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap5(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 5);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap6(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 6);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap7(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 7);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap8(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 8);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap9(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 9);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap10(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 10);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap11(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 11);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap12(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 12);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap13(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 13);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap14(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 14);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap15(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 15);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_swap16(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::swap(state, 16);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_return(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::ret(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_revert(state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = self::misc::revert(state);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_invalid(_state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = Control::Exit(ExitError::DesignatedInvalid.into());
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

fn eval_external(_state: &mut Machine, opcode: Opcode, _position: usize) -> Control {
	unsafe { gas_profile_start() }
	let res = Control::Trap(opcode);
	unsafe { gas_profile_stop(opcode.0 as u32) }
	res
}

#[inline]
#[cfg(FALSE)]
pub fn eval(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	static TABLE: [fn(state: &mut Machine, opcode: Opcode, position: usize) -> Control; 256] = {
		let mut table = [eval_external as _; 256];

		table[Opcode::STOP.as_usize()] = eval_stop as _;
		table[Opcode::ADD.as_usize()] = eval_add as _;
		table[Opcode::MUL.as_usize()] = eval_mul as _;
		table[Opcode::SUB.as_usize()] = eval_sub as _;
		table[Opcode::DIV.as_usize()] = eval_div as _;
		table[Opcode::SDIV.as_usize()] = eval_sdiv as _;
		table[Opcode::MOD.as_usize()] = eval_mod as _;
		table[Opcode::SMOD.as_usize()] = eval_smod as _;
		table[Opcode::ADDMOD.as_usize()] = eval_addmod as _;
		table[Opcode::MULMOD.as_usize()] = eval_mulmod as _;
		table[Opcode::EXP.as_usize()] = eval_exp as _;
		table[Opcode::SIGNEXTEND.as_usize()] = eval_signextend as _;
		table[Opcode::LT.as_usize()] = eval_lt as _;
		table[Opcode::GT.as_usize()] = eval_gt as _;
		table[Opcode::SLT.as_usize()] = eval_slt as _;
		table[Opcode::SGT.as_usize()] = eval_sgt as _;
		table[Opcode::EQ.as_usize()] = eval_eq as _;
		table[Opcode::ISZERO.as_usize()] = eval_iszero as _;
		table[Opcode::AND.as_usize()] = eval_and as _;
		table[Opcode::OR.as_usize()] = eval_or as _;
		table[Opcode::XOR.as_usize()] = eval_xor as _;
		table[Opcode::NOT.as_usize()] = eval_not as _;
		table[Opcode::BYTE.as_usize()] = eval_byte as _;
		table[Opcode::SHL.as_usize()] = eval_shl as _;
		table[Opcode::SHR.as_usize()] = eval_shr as _;
		table[Opcode::SAR.as_usize()] = eval_sar as _;
		table[Opcode::CODESIZE.as_usize()] = eval_codesize as _;
		table[Opcode::CODECOPY.as_usize()] = eval_codecopy as _;
		table[Opcode::CALLDATALOAD.as_usize()] = eval_calldataload as _;
		table[Opcode::CALLDATASIZE.as_usize()] = eval_calldatasize as _;
		table[Opcode::CALLDATACOPY.as_usize()] = eval_calldatacopy as _;
		table[Opcode::POP.as_usize()] = eval_pop as _;
		table[Opcode::MLOAD.as_usize()] = eval_mload as _;
		table[Opcode::MSTORE.as_usize()] = eval_mstore as _;
		table[Opcode::MSTORE8.as_usize()] = eval_mstore8 as _;
		table[Opcode::JUMP.as_usize()] = eval_jump as _;
		table[Opcode::JUMPI.as_usize()] = eval_jumpi as _;
		table[Opcode::PC.as_usize()] = eval_pc as _;
		table[Opcode::MSIZE.as_usize()] = eval_msize as _;
		table[Opcode::JUMPDEST.as_usize()] = eval_jumpdest as _;

		table[Opcode::PUSH1.as_usize()] = eval_push1 as _;
		table[Opcode::PUSH2.as_usize()] = eval_push2 as _;
		table[Opcode::PUSH3.as_usize()] = eval_push3 as _;
		table[Opcode::PUSH4.as_usize()] = eval_push4 as _;
		table[Opcode::PUSH5.as_usize()] = eval_push5 as _;
		table[Opcode::PUSH6.as_usize()] = eval_push6 as _;
		table[Opcode::PUSH7.as_usize()] = eval_push7 as _;
		table[Opcode::PUSH8.as_usize()] = eval_push8 as _;
		table[Opcode::PUSH9.as_usize()] = eval_push9 as _;
		table[Opcode::PUSH10.as_usize()] = eval_push10 as _;
		table[Opcode::PUSH11.as_usize()] = eval_push11 as _;
		table[Opcode::PUSH12.as_usize()] = eval_push12 as _;
		table[Opcode::PUSH13.as_usize()] = eval_push13 as _;
		table[Opcode::PUSH14.as_usize()] = eval_push14 as _;
		table[Opcode::PUSH15.as_usize()] = eval_push15 as _;
		table[Opcode::PUSH16.as_usize()] = eval_push16 as _;
		table[Opcode::PUSH17.as_usize()] = eval_push17 as _;
		table[Opcode::PUSH18.as_usize()] = eval_push18 as _;
		table[Opcode::PUSH19.as_usize()] = eval_push19 as _;
		table[Opcode::PUSH20.as_usize()] = eval_push20 as _;
		table[Opcode::PUSH21.as_usize()] = eval_push21 as _;
		table[Opcode::PUSH22.as_usize()] = eval_push22 as _;
		table[Opcode::PUSH23.as_usize()] = eval_push23 as _;
		table[Opcode::PUSH24.as_usize()] = eval_push24 as _;
		table[Opcode::PUSH25.as_usize()] = eval_push25 as _;
		table[Opcode::PUSH26.as_usize()] = eval_push26 as _;
		table[Opcode::PUSH27.as_usize()] = eval_push27 as _;
		table[Opcode::PUSH28.as_usize()] = eval_push28 as _;
		table[Opcode::PUSH29.as_usize()] = eval_push29 as _;
		table[Opcode::PUSH30.as_usize()] = eval_push30 as _;
		table[Opcode::PUSH31.as_usize()] = eval_push31 as _;
		table[Opcode::PUSH32.as_usize()] = eval_push32 as _;

		table[Opcode::DUP1.as_usize()] = eval_dup1 as _;
		table[Opcode::DUP2.as_usize()] = eval_dup2 as _;
		table[Opcode::DUP3.as_usize()] = eval_dup3 as _;
		table[Opcode::DUP4.as_usize()] = eval_dup4 as _;
		table[Opcode::DUP5.as_usize()] = eval_dup5 as _;
		table[Opcode::DUP6.as_usize()] = eval_dup6 as _;
		table[Opcode::DUP7.as_usize()] = eval_dup7 as _;
		table[Opcode::DUP8.as_usize()] = eval_dup8 as _;
		table[Opcode::DUP9.as_usize()] = eval_dup9 as _;
		table[Opcode::DUP10.as_usize()] = eval_dup10 as _;
		table[Opcode::DUP11.as_usize()] = eval_dup11 as _;
		table[Opcode::DUP12.as_usize()] = eval_dup12 as _;
		table[Opcode::DUP13.as_usize()] = eval_dup13 as _;
		table[Opcode::DUP14.as_usize()] = eval_dup14 as _;
		table[Opcode::DUP15.as_usize()] = eval_dup15 as _;
		table[Opcode::DUP16.as_usize()] = eval_dup16 as _;

		table[Opcode::SWAP1.as_usize()] = eval_swap1 as _;
		table[Opcode::SWAP2.as_usize()] = eval_swap2 as _;
		table[Opcode::SWAP3.as_usize()] = eval_swap3 as _;
		table[Opcode::SWAP4.as_usize()] = eval_swap4 as _;
		table[Opcode::SWAP5.as_usize()] = eval_swap5 as _;
		table[Opcode::SWAP6.as_usize()] = eval_swap6 as _;
		table[Opcode::SWAP7.as_usize()] = eval_swap7 as _;
		table[Opcode::SWAP8.as_usize()] = eval_swap8 as _;
		table[Opcode::SWAP9.as_usize()] = eval_swap9 as _;
		table[Opcode::SWAP10.as_usize()] = eval_swap10 as _;
		table[Opcode::SWAP11.as_usize()] = eval_swap11 as _;
		table[Opcode::SWAP12.as_usize()] = eval_swap12 as _;
		table[Opcode::SWAP13.as_usize()] = eval_swap13 as _;
		table[Opcode::SWAP14.as_usize()] = eval_swap14 as _;
		table[Opcode::SWAP15.as_usize()] = eval_swap15 as _;
		table[Opcode::SWAP16.as_usize()] = eval_swap16 as _;

		table[Opcode::RETURN.as_usize()] = eval_return as _;
		table[Opcode::REVERT.as_usize()] = eval_revert as _;
		table[Opcode::INVALID.as_usize()] = eval_invalid as _;

		table
	};

	TABLE[opcode.as_usize()](state, opcode, position)
}

// 118815126965304
// 110436615921984
// 110741289050832
#[inline]
pub fn eval(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	(match opcode {
		Opcode::STOP => eval_stop,
		Opcode::ADD => eval_add,
		Opcode::MUL => eval_mul,
		Opcode::SUB => eval_sub,
		Opcode::DIV => eval_div,
		Opcode::SDIV => eval_sdiv,
		Opcode::MOD => eval_mod,
		Opcode::SMOD => eval_smod,
		Opcode::ADDMOD => eval_addmod,
		Opcode::MULMOD => eval_mulmod,
		Opcode::EXP => eval_exp,
		Opcode::SIGNEXTEND => eval_signextend,
		Opcode::LT => eval_lt,
		Opcode::GT => eval_gt,
		Opcode::SLT => eval_slt,
		Opcode::SGT => eval_sgt,
		Opcode::EQ => eval_eq,
		Opcode::ISZERO => eval_iszero,
		Opcode::AND => eval_and,
		Opcode::OR => eval_or,
		Opcode::XOR => eval_xor,
		Opcode::NOT => eval_not,
		Opcode::BYTE => eval_byte,
		Opcode::SHL => eval_shl,
		Opcode::SHR => eval_shr,
		Opcode::SAR => eval_sar,
		Opcode::CODESIZE => eval_codesize,
		Opcode::CODECOPY => eval_codecopy,
		Opcode::CALLDATALOAD => eval_calldataload,
		Opcode::CALLDATASIZE => eval_calldatasize,
		Opcode::CALLDATACOPY => eval_calldatacopy,
		Opcode::POP => eval_pop,
		Opcode::MLOAD => eval_mload,
		Opcode::MSTORE => eval_mstore,
		Opcode::MSTORE8 => eval_mstore8,
		Opcode::JUMP => eval_jump,
		Opcode::JUMPI => eval_jumpi,
		Opcode::PC => eval_pc,
		Opcode::MSIZE => eval_msize,
		Opcode::JUMPDEST => eval_jumpdest,

		Opcode::PUSH1 => eval_push1,
		Opcode::PUSH2 => eval_push2,
		Opcode::PUSH3 => eval_push3,
		Opcode::PUSH4 => eval_push4,
		Opcode::PUSH5 => eval_push5,
		Opcode::PUSH6 => eval_push6,
		Opcode::PUSH7 => eval_push7,
		Opcode::PUSH8 => eval_push8,
		Opcode::PUSH9 => eval_push9,
		Opcode::PUSH10 => eval_push10,
		Opcode::PUSH11 => eval_push11,
		Opcode::PUSH12 => eval_push12,
		Opcode::PUSH13 => eval_push13,
		Opcode::PUSH14 => eval_push14,
		Opcode::PUSH15 => eval_push15,
		Opcode::PUSH16 => eval_push16,
		Opcode::PUSH17 => eval_push17,
		Opcode::PUSH18 => eval_push18,
		Opcode::PUSH19 => eval_push19,
		Opcode::PUSH20 => eval_push20,
		Opcode::PUSH21 => eval_push21,
		Opcode::PUSH22 => eval_push22,
		Opcode::PUSH23 => eval_push23,
		Opcode::PUSH24 => eval_push24,
		Opcode::PUSH25 => eval_push25,
		Opcode::PUSH26 => eval_push26,
		Opcode::PUSH27 => eval_push27,
		Opcode::PUSH28 => eval_push28,
		Opcode::PUSH29 => eval_push29,
		Opcode::PUSH30 => eval_push30,
		Opcode::PUSH31 => eval_push31,
		Opcode::PUSH32 => eval_push32,

		Opcode::DUP1 => eval_dup1,
		Opcode::DUP2 => eval_dup2,
		Opcode::DUP3 => eval_dup3,
		Opcode::DUP4 => eval_dup4,
		Opcode::DUP5 => eval_dup5,
		Opcode::DUP6 => eval_dup6,
		Opcode::DUP7 => eval_dup7,
		Opcode::DUP8 => eval_dup8,
		Opcode::DUP9 => eval_dup9,
		Opcode::DUP10 => eval_dup10,
		Opcode::DUP11 => eval_dup11,
		Opcode::DUP12 => eval_dup12,
		Opcode::DUP13 => eval_dup13,
		Opcode::DUP14 => eval_dup14,
		Opcode::DUP15 => eval_dup15,
		Opcode::DUP16 => eval_dup16,

		Opcode::SWAP1 => eval_swap1,
		Opcode::SWAP2 => eval_swap2,
		Opcode::SWAP3 => eval_swap3,
		Opcode::SWAP4 => eval_swap4,
		Opcode::SWAP5 => eval_swap5,
		Opcode::SWAP6 => eval_swap6,
		Opcode::SWAP7 => eval_swap7,
		Opcode::SWAP8 => eval_swap8,
		Opcode::SWAP9 => eval_swap9,
		Opcode::SWAP10 => eval_swap10,
		Opcode::SWAP11 => eval_swap11,
		Opcode::SWAP12 => eval_swap12,
		Opcode::SWAP13 => eval_swap13,
		Opcode::SWAP14 => eval_swap14,
		Opcode::SWAP15 => eval_swap15,
		Opcode::SWAP16 => eval_swap16,

		Opcode::RETURN => eval_return,
		Opcode::REVERT => eval_revert,
		Opcode::INVALID => eval_invalid,
		_ => eval_external,
	}(state, opcode, position))
}
