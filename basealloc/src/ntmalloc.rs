use std::alloc::{GlobalAlloc, Layout};
use crate::errno::Errno;
use crate::error::BaseRegionError;
use libc;

// provide base posix memory manager interface
// also with rust style
pub trait PosixMemRust{
    fn malloc(&self, size: usize) -> Result<*mut u8, Errno>;
    fn calloc(&self, nelem: usize, elesize: usize) -> Result<*mut u8, Errno>;
    fn realloc(&self, addr: *mut u8, size: usize) -> Result<*mut u8, Errno>;
    fn reallocarray(&self, addr: *mut u8, nelem: usize, elesize: usize) -> Result<*mut u8, Errno>;
    fn free(&self, addr: *mut u8);
}

pub struct Ntmalloc;

impl PosixMemRust for Ntmalloc {
    fn malloc(&self, size: usize) -> Result<*mut u8, Errno> {
        // FIXME: use origin Linux syscall instead C Library to implement
        // See the diff of "brk" between Linux/C library
        let cur_brk = libc::sbrk(0);
        if cur_brk as i32 == -1 {
            println!("call sbrk failed");
            return Err(Errno::last());
        }

    }
}

pub enum BaseRegionFg {
    Normal,
}

pub enum BaseRegionState{
    Using,
    Free,
}

pub struct BaseAllocRegion {
    base: u64,
    size: u64,
    state: BaseRegionState,
    flag: BaseRegionFg,
}

const BASE_REGION_NUMS: usize = 128;
static mut BASE_REGIONS: [BaseAllocRegion; BASE_REGION_NUMS];
pub struct BaseAlloc {
    cnt: u32,
    max: u32,
    total_size: u64,
    next_region_idx: u32,
    region_ptr: *mut BaseAllocRegion,
}

// tauch the syscall
pub trait BaseTrait {
    fn add_region(&self, new_region: BaseAllocRegion) -> Result<u32, BaseRegionError>;
    fn free_region_base(&self, base: u64) -> Result<u32, BaseRegionError>;
    fn get_region_base(&self, base: u64) -> Result<BaseAllocRegion, BaseRegionError>;
    fn free_region_idx(&self, idx: u32) -> Result<u32, BaseRegionError>;
    fn get_region_idx(&self, idx: u32) -> Result<BaseAllocRegion, BaseRegionError>;
    fn get_free_region_base(&self, base: u64) -> Option<BaseAllocRegion>;
    fn get_free_region_idx(&self, idx: u32) -> Option<BaseAllocRegion>;
    // fn _merge_cur_region(cur_region: BaseAllocRegion) -> Option<u32>
    // try to merge cur region, if success ret the merged BaseAllocRegion idx
}

impl BaseAllocRegion {
    //fn _size_align
    pub fn new(size:u64) -> Result<Self, BaseRegionError>{
        if size&0x8 != 0 {
            let temp_val: u64 = 0x8;
            size = size&(!temp_val) + 0x8;
            println!("size = 0x{:<16x}", size);
        }
        // TODO: add ret check
        let cur_brk = libc::sbrk(size);
        if cur_brk as i64 == -1 {
            return Err(BaseRegionError::ENOMEM);
        }
        Ok(BaseAllocRegion{base: cur_brk as u64, size: size, state: BaseRegionState::Using, flag: BaseRegionFg::Normal})
    }

}


unsafe impl GlobalAlloc for Ntmalloc{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8{
        
    }
}

