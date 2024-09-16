//! Collections of programs used exclusively by the official JSON-RPC specification.

mod bump_allocator;
mod program_error;
pub mod spl_token;
mod syscall_stubs;
mod syscalls;
pub mod system;

use crate::blockchain::solana::ProgramAccount;
use alloc::{rc::Rc, vec::Vec};
pub use bump_allocator::BumpAllocator;
use core::{cell::RefCell, num::Saturating, ops::Sub, panic::PanicInfo, slice};
pub use program_error::ProgramError;
pub use syscall_stubs::SyscallStubs;
pub use syscalls::*;

/// Program success
pub const SUCCESS: u64 = 0;
/// Start address of the memory region used for program heap.
pub const HEAP_START_ADDRESS: u64 = 0x300000000;
/// Length of the heap memory region used for program heap.
pub const HEAP_LENGTH: usize = 32 * 1024;

const BPF_ALIGN_OF_U128: usize = 8;
const MAX_PERMITTED_DATA_INCREASE: usize = 1_024 * 10;
const NON_DUP_MARKER: u8 = u8::MAX;
const UNSUPPORTED_SYSVAR: u64 = to_builtin(17);

/// Default panic handler used in entrypoints.
#[inline]
pub fn default_panic_handler(info: &PanicInfo<'_>) {
  if let Some(elem) = info.message().as_str() {
    log(elem)
  } else {
    log(&alloc::format!("{}", info))
  }
}

/// Deserializes the input of a program's entrypoint.
pub unsafe fn deserialize<'any>(
  input: *mut u8,
) -> (&'any [u8; 32], Vec<ProgramAccount<'any>>, &'any [u8]) {
  let mut offset: usize = 0;

  // Number of accounts present

  let num_accounts = unsafe { *(input.add(offset) as *const u64) as usize };
  offset += size_of::<u64>();

  // Account Infos

  let mut accounts = Vec::with_capacity(num_accounts);
  for _ in 0..num_accounts {
    let dup_info = unsafe { *(input.add(offset) as *const u8) };
    offset += size_of::<u8>();
    if dup_info == NON_DUP_MARKER {
      let is_signer = unsafe { *(input.add(offset) as *const u8) != 0 };
      offset += size_of::<u8>();

      let is_writable = unsafe { *(input.add(offset) as *const u8) != 0 };
      offset += size_of::<u8>();

      let executable = unsafe { *(input.add(offset) as *const u8) != 0 };
      offset += size_of::<u8>();

      // The original data length is stored here because these 4 bytes were
      // originally only used for padding and served as a good location to
      // track the original size of the account data in a compatible way.
      let original_data_len_offset = offset;
      offset += size_of::<u32>();

      let key: &[u8; 32] = unsafe { &*(input.add(offset) as *const [u8; 32]) };
      offset += size_of::<[u8; 32]>();

      let owner: &[u8; 32] = unsafe { &*(input.add(offset) as *const [u8; 32]) };
      offset += size_of::<[u8; 32]>();

      let lamports = unsafe { Rc::new(RefCell::new(&mut *(input.add(offset) as *mut u64))) };
      offset += size_of::<u64>();

      let data_len = unsafe { *(input.add(offset) as *const u64) as usize };
      offset += size_of::<u64>();

      // Store the original data length for detecting invalid reallocations and
      // requires that MAX_PERMITTED_DATA_LENGTH fits in a u32
      unsafe {
        *(input.add(original_data_len_offset) as *mut u32) = data_len as u32;
      }

      let data =
        Rc::new(RefCell::new(unsafe { slice::from_raw_parts_mut(input.add(offset), data_len) }));
      offset += data_len + MAX_PERMITTED_DATA_INCREASE;
      offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128); // padding

      let rent_epoch = unsafe { *(input.add(offset) as *const u64) };
      offset += size_of::<u64>();

      accounts.push(ProgramAccount {
        key,
        is_signer,
        is_writable,
        lamports,
        data,
        owner,
        executable,
        rent_epoch,
      });
    } else {
      offset += 7; // padding

      // Duplicate account, clone the original
      accounts.push(accounts[dup_info as usize].clone());
    }
  }

  // Instruction data

  let instruction_data_len = unsafe { *(input.add(offset) as *const u64) as usize };
  offset += size_of::<u64>();

  let instruction_data = unsafe { slice::from_raw_parts(input.add(offset), instruction_data_len) };
  offset += instruction_data_len;

  // Program Id

  let program_id: &[u8; 32] = unsafe { &*(input.add(offset) as *const [u8; 32]) };

  (program_id, accounts, instruction_data)
}

/// Checks if two regions do not overlap.
pub fn is_nonoverlapping<T>(src: T, src_len: T, dst: T, dst_len: T) -> bool
where
  T: Ord,
  Saturating<T>: Sub<Saturating<T>, Output = Saturating<T>>,
{
  if &src > &dst {
    Saturating(src).sub(Saturating(dst)).0 >= dst_len
  } else {
    Saturating(dst).sub(Saturating(src)).0 >= src_len
  }
}

/// Prints a string to the log.
#[inline]
pub fn log(_message: &str) {
  #[cfg(target_os = "solana")]
  unsafe {
    crate::solana::program::sol_log_(_message.as_ptr(), _message.len() as u64);
  }
  #[cfg(all(feature = "std", not(target_os = "solana")))]
  {
    use alloc::boxed::Box;
    use std::sync::{LazyLock, RwLock};

    type Ty = LazyLock<RwLock<Box<dyn SyscallStubs>>>;

    struct DefaultSyscallStubs {}
    impl SyscallStubs for DefaultSyscallStubs {}

    static SYSCALL_STUBS: Ty = LazyLock::new(|| RwLock::new(Box::new(DefaultSyscallStubs {})));
    SYSCALL_STUBS.read().unwrap().sol_log(_message);
  }
}

#[inline]
const fn to_builtin(n: u64) -> u64 {
  n << 32
}
