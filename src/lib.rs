

#[derive(Default)]
pub struct Syscall {}

impl Syscall {
    #[inline]
    fn syscall(&mut self) {
        panic!("Crypto service needs to be supplied with a platform specific method to syscall.");
    }
}
