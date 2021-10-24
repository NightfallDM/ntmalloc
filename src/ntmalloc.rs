use std::alloc::{GlobalAlloc, Layout};
use errno::Errno;
pub use libc;

// provide base posix memory manager interface
// also with rust style
pub trait PosixMemRust{
    pub fn malloc(&self, size: usize) -> Result<*mut u8, Errno>;
    pub fn calloc(&self, nelem: usize, elesize: usize) -> Result<*mut u8, Errno>;
    pub fn realloc(&self, addr: *mut u8, size: usize) -> Result<*mut u8, Errno>;
    pub fn reallocarray(&self, addr *mut u8, nelem: usize, elesize: usize) -> Result<*mut u8, Errno>;
    pub fn free(&self, addr: *mut u8);
}

pub struct Ntmalloc;

impl PosixMenRust for Ntmalloc {
    pub fn malloc(&self, size: usize) -> Result<*mut u8, Errno> {
        // FIXME: use origin Linux syscall instead C Library to implement
        // See the diff of "brk" between Linux/C library
        let cur_brk = libc::sbrk(0);
        if (cur_brk as i32 == -1){
            println!("call sbrk failed");
            return Err(Errno::last());
        }

    }
}

pub enum BaseAllocRegionFg {
    
}


pub struct BaseAllocRegion {
    base:   u64,
    size:   u64,
    flag:   BaseAllocRegionFg,
}

const static BASE_REGION_NUMS = 128;
static mut BASE_REGIONS: [BaseAllocRegion; BASE_REGION_NUMS];
pub struct BaseAlloc {

}


unsafe impl GlobalAlloc for Ntmalloc{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8{
        
    }
}

