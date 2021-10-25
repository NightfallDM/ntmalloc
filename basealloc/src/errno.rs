// list the errno
use libc;

#[repr(i32)]
pub enum Errno {
    // brk, sbrk
    ENOMEM = libc::ENOMEM,
    // end brk, sbrk

    // mmap
    EACCESS = libc::EACCES,
    EAGAIN = libc::EAGAIN,
    EBADF = libc::EBADF,
    EEXIST = libc::EEXIST,
    EINVAL = libc::EINVAL,
    ENFILE = libc::ENFILE,
    ENODEV = libc::ENODEV,
    /*
    *ENOMEM
    * */
    EOVERFLOW = libc::EOVERFLOW,
    EPERM = libc::EPERM,
    ETXTBSY = libc::ETXTBSY,
    // end mmap
}
