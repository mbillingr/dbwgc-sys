use dbwgc_sys::{DbwGcAllocator, GC_init, GC_is_init_called, GC_malloc, GC_free, GC_get_free_bytes, GC_get_total_bytes};

#[global_allocator]
static A: DbwGcAllocator = DbwGcAllocator;

fn main() {
    unsafe {
        GC_init();
        assert_ne!(GC_is_init_called(), 0);

        for i in 0..1000000 {
            let x = Box::new(vec![i; 1024*1024]);
            let _ = Box::leak(x);
        }

        println!("free: {}", GC_get_free_bytes());
        println!("total: {}", GC_get_total_bytes());

        //GC_deinit();
    }
}
