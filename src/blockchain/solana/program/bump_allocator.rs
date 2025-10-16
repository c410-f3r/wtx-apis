use alloc::alloc::GlobalAlloc;
use core::{alloc::Layout, ptr};

/// Updates the pointer
#[derive(Debug)]
pub struct BumpAllocator {
  /// Start
  pub start: usize,
  /// Length
  pub len: usize,
}

unsafe impl GlobalAlloc for BumpAllocator {
  #[inline]
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let pos_ptr = self.start as *mut usize;
    let mut pos = unsafe { *pos_ptr };
    if pos == 0 {
      pos = self.start.wrapping_add(self.len);
    }
    pos = pos.saturating_sub(layout.size());
    pos &= !(layout.align().wrapping_sub(1));
    if pos < self.start.wrapping_add(size_of::<*mut u8>()) {
      return ptr::null_mut();
    }
    unsafe {
      *pos_ptr = pos;
    }
    pos as *mut u8
  }

  #[inline]
  unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}
