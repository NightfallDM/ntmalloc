use libc;

fn main() {
    unsafe{
        let cur_brk = libc::sbrk(0);
        println!("cur_brk = 0x{:<16x}", cur_brk as usize);
        let addr = cur_brk as usize + (1 << 12);
        println!("addr = 0x{:<16x}", addr);
        let ret = libc::brk(addr as *mut libc::c_void);
        if let 0 = ret{
            println!("modify break finish");
            println!("cur_brk = 0x{:<16x}", libc::sbrk(0) as usize);
        }else {
            println!("ret = {}", ret as i32);
        }
    }
    println!("Hello, world!");
}
