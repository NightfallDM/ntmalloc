use std::alloc::{GlobalAlloc, Layout};
use libc;

pub struct Ntmalloc;

unsafe impl GlobalAlloc for Ntmalloc{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8{
        
    }
}

