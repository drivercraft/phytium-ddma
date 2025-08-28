use core::{hint::spin_loop, ptr::NonNull};

use dma_api::DVec;
use log::trace;
use tock_registers::interfaces::*;

use crate::reg::*;

pub struct Channel {
    n: u8,
    reg: NonNull<DmaChannelRegisters>,
    buff: DVec<u8>,
}

unsafe impl Send for Channel {}

#[derive(Debug, Clone)]
pub struct ChannelConfig {
    pub slave_id: u8,
    pub direction: crate::DmaDirection,
    pub timeout_count: u32,
    pub blk_size: usize,
    pub dev_addr: u32,
}

impl Channel {
    pub(crate) fn new(
        n: u8,
        reg: NonNull<DmaChannelRegisters>,
        config: ChannelConfig,
    ) -> Option<Self> {
        let mut s = Self {
            n,
            reg,
            buff: DVec::zeros(config.blk_size, 128, dma_api::Direction::Bidirectional)?,
        };
        let ddr = s.buff.bus_addr();

        // Check DDR address alignment (following C reference)
        if !ddr.is_multiple_of(4) {
            trace!("DDR addr 0x{:x} must be aligned with 4 bytes.", ddr);
            return None;
        }

        // Check transfer size alignment (following C reference)
        if config.blk_size < 4 || !config.blk_size.is_multiple_of(4) {
            trace!(
                "Invalid transfer size {} bytes, it should be an integer multiple of 4 bytes.",
                config.blk_size
            );
            return None;
        }

        if s.reg().ctl.is_set(DMA_CHALX_CTL::CHALX_EN) {
            s.reset();
        }

        s.reg().ddr_lwaddr.set((ddr & 0xFFFF_FFFF) as u32);
        s.reg().ddr_upaddr.set((ddr >> 32) as u32);
        s.reg().dev_addr.set(config.dev_addr);
        s.reg().ts.set(config.blk_size as u32);

        s.reg().ctl.modify(match config.direction {
            crate::DmaDirection::MemoryToDevice => DMA_CHALX_CTL::CHALX_MODE::Tx,
            crate::DmaDirection::DeviceToMemory => DMA_CHALX_CTL::CHALX_MODE::Rx,
        });

        Some(s)
    }

    pub fn index(&self) -> u8 {
        self.n
    }

    fn reset(&mut self) {
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_EN::CLEAR);
        while self.reg().ctl.is_set(DMA_CHALX_CTL::CHALX_EN) {
            spin_loop();
        }
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_SRST::SET);
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_SRST::CLEAR);
        trace!("Channel {} reset done", self.n);
    }

    pub fn active(&mut self) {
        // Clear any pending interrupts first (following C reference)
        // Note: This would need to be done at the controller level
        // but we can clear channel-specific status here
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_EN::SET);
        trace!("Channel {} activated", self.n);
    }

    pub fn clear_and_active(&mut self, dma: &mut crate::DDMA) {
        // Clear pending interrupts at controller level (following C reference)
        dma.clear_transfer_complete(self.n);
        self.active();
    }

    pub fn deactive(&mut self) {
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_EN::CLEAR);
    }

    fn reg(&self) -> &DmaChannelRegisters {
        unsafe { self.reg.as_ref() }
    }

    pub fn buff(&self) -> &DVec<u8> {
        &self.buff
    }

    pub fn buff_mut(&mut self) -> &mut DVec<u8> {
        &mut self.buff
    }
}
