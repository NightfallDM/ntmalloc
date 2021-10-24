// list the errno
use libc;

pub enum Errno {
    // brk, sbrk
    ENOMEM: libc::ENOMEM,
    // end brk, sbrk

    // mmap
    EACCES: libc::EACCES,
    EAGAIN: lic::EAGAIN,
    EBADF:  libc::EBADF,
    EEXIST: libc::EEXIST,
    EINVAL: libc::EINVAL,
    ENFILE: libc::ENFILE,
    ENODEV: libc::ENODEV,
    /*
    *ENOMEM
    * */
    EOVERFLOW:  libc::EOVERFLOW,
    EPERM:  libc::EPERM,
    ETXTBSY:    libc::ETXTBSY,
    // end mmap
}
