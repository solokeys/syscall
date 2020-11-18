#![no_std]

#[derive(Default)]
pub struct Syscall {}

impl Syscall {
    #[inline]
    pub fn syscall(&mut self) {
        panic!("Crypto service needs to be supplied with a platform specific method to syscall.");
    }
}
