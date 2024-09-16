/// Starting point of Solana programs.
///
/// This macro will trigger UB if called outside of a solana program.
#[macro_export]
macro_rules! entrypoint {
  ($process_instruction:ident) => {
    #[global_allocator]
    static A: $crate::blockchain::solana::program::BumpAllocator =
      $crate::blockchain::solana::program::BumpAllocator {
        start: $crate::blockchain::solana::program::HEAP_START_ADDRESS as usize,
        len: $crate::blockchain::solana::program::HEAP_LENGTH,
      };

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
      let tuple = unsafe { $crate::blockchain::solana::program::deserialize(input) };
      let (program_id, accounts, ix_data) = tuple;
      match $process_instruction(&program_id, &accounts, &ix_data) {
        Err(error) => error.into(),
        Ok(()) => $crate::blockchain::solana::program::SUCCESS,
      }
    }

    #[unsafe(no_mangle)]
    fn custom_panic(info: &core::panic::PanicInfo<'_>) {
      $crate::blockchain::solana::program::default_panic_handler(info);
    }
  };
}

#[macro_export]
macro_rules! msg {
  ($msg:expr) => { $crate::blockchain::solana::program::log($msg) };
  ($($arg:tt)*) => ($crate::blockchain::solana::program::log(&alloc::format!($($arg)*)));
}

macro_rules! create_syscall {
	(fn $name:ident($($arg:ident: $typ:ty),*) -> $ret:ty) => {
		unsafe extern "C" {
			pub fn $name($($arg: $typ),*) -> $ret;
		}
	};
	(fn $name:ident($($arg:ident: $typ:ty),*)) => {
		create_syscall!(fn $name($($arg: $typ),*) -> ());
	}
}

macro_rules! generic_config_doc {
  () => {
    "Additional set of optional parameters used by the corresponding request."
  };
}

macro_rules! commitment_doc {
  () => {
    "Additional set of optional parameters used by the corresponding request."
  };
}

macro_rules! min_context_slot_doc {
  () => {
    "Set the minimum slot for the request."
  };
}
