mod gc;

pub use gc::*;

#[link(name = "gc")]
extern {}

use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;

pub struct DbwGcAllocator;

unsafe impl GlobalAlloc for DbwGcAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = GC_malloc(layout.size()) as *mut _;
        debug_assert_eq!(ptr as usize % layout.align(), 0);
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}
