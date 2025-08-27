#![no_std]
#![recursion_limit = "512"]

use core::ptr::NonNull;

extern crate alloc;

pub mod reg;

pub struct DDMA {
    reg: NonNull<reg::DdmaRegister>,
}

impl DDMA {
    pub fn new(base_addr: NonNull<u8>) -> Self {
        Self {
            reg: base_addr.cast(),
        }
    }

    pub fn irq_handler(&self) -> IrqHandler {
        IrqHandler { reg: self.reg }
    }
}

pub struct IrqHandler {
    reg: NonNull<reg::DdmaRegister>,
}

unsafe impl Send for IrqHandler {}
unsafe impl Sync for IrqHandler {}

impl IrqHandler {
    pub fn handle_irq(&self) {
        // TODO
    }
}
