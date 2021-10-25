use libc;

#[repr(i32)]
pub enum BaseRegionError{
    ENOMEM = libc::ENOMEM,
}
