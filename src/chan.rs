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
        self.reg().ctl.modify(DMA_CHALX_CTL::CHALX_EN::SET);
        trace!("Channel {} activated", self.n);
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
