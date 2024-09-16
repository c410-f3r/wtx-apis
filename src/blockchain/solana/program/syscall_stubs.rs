use crate::blockchain::solana::{
  program::{is_nonoverlapping, program_error::ProgramError, UNSUPPORTED_SYSVAR},
  InstructionInput, ProgramAccount,
};
use alloc::vec::Vec;
use core::{ptr, slice};

pub trait SyscallStubs: Sync + Send {
  fn sol_invoke_signed(
    &self,
    _instruction: &InstructionInput,
    _account_infos: &[ProgramAccount<'_>],
    _signers_seeds: &[&[&[u8]]],
  ) -> Result<(), ProgramError> {
    Ok(())
  }

  fn sol_log(&self, _message: &str) {
    #[cfg(feature = "std")]
    std::println!("{_message}");
  }

  fn sol_log_compute_units(&self) {}

  fn sol_remaining_compute_units(&self) -> u64 {
    0
  }

  fn sol_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_epoch_rewards_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_last_restart_slot(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_processed_sibling_instruction(&self, _index: usize) -> Option<InstructionInput> {
    None
  }

  fn sol_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
  }

  fn sol_get_return_data(&self) -> Option<([u8; 32], Vec<u8>)> {
    None
  }

  fn sol_get_stack_height(&self) -> u64 {
    0
  }

  fn sol_log_data(&self, _fields: &[&[u8]]) {
    #[cfg(feature = "std")]
    std::println!("data: {:?}", _fields);
  }

  /// # Safety
  unsafe fn sol_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
    let mut i = 0;
    while i < n {
      let a = unsafe { *s1.add(i) };
      let b = unsafe { *s2.add(i) };
      if a != b {
        unsafe {
          *result = a as i32 - b as i32;
        }
        return;
      }
      i += 1;
    }
    unsafe { *result = 0 }
  }

  unsafe fn sol_memcpy(&self, dst: *mut u8, src: *const u8, n: usize) {
    assert!(
      is_nonoverlapping(src as usize, n, dst as usize, n),
      "memcpy does not support overlapping regions"
    );
    unsafe {
      ptr::copy_nonoverlapping(src, dst, n);
    }
  }

  unsafe fn sol_memmove(&self, dst: *mut u8, src: *const u8, n: usize) {
    unsafe {
      ptr::copy(src, dst, n);
    }
  }

  unsafe fn sol_memset(&self, s: *mut u8, c: u8, n: usize) {
    let s = unsafe { slice::from_raw_parts_mut(s, n) };
    for val in s.iter_mut().take(n) {
      *val = c;
    }
  }

  fn sol_set_return_data(&self, _data: &[u8]) {}
}
