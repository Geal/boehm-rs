extern crate boehm;

use boehm::ffi;
use std::libc;

// a straight port of http://www.hpl.hp.com/personal/Hans_Boehm/gc/simple_example.html

#[start]
fn main(_: int, _: **u8) -> int {
    unsafe {
        ffi::GC_init();

        for i in range(0, 10000000) {
            let p = ffi::GC_malloc(8) as *mut *i64;
            let q = ffi::GC_malloc_atomic(8) as *i64;
            assert!((*p).is_null());
            *p = ffi::GC_realloc(q as *mut libc::c_void, 16) as *i64;
            if i % 100000 == 0 {
                println!("Heap size = {}", ffi::GC_get_heap_size());
            }
        }
    }

    0
}
