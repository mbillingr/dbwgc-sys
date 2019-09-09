use dbwgc_sys::{GC_init, GC_is_init_called, GC_deinit, GC_malloc, GC_free, GC_get_free_bytes, GC_get_total_bytes};

use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;

struct DbwGcAllocator;

unsafe impl GlobalAlloc for DbwGcAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = GC_malloc(layout.size()) as *mut _;
        debug_assert_eq!(ptr as usize % layout.align(), 0);
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}

#[global_allocator]
static A: DbwGcAllocator = DbwGcAllocator;

fn main() {
    unsafe {
        GC_init();
        assert_ne!(GC_is_init_called(), 0);

        for i in 0..1000000 {
            let x = Box::new(vec![i; 1024*1024]);
            let x = Box::leak(x);
        }

        println!("free: {}", GC_get_free_bytes());
        println!("total: {}", GC_get_total_bytes());

        GC_deinit();
    }
}
