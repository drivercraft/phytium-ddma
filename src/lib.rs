#![no_std]
#![recursion_limit = "512"]

use core::ptr::NonNull;
use log::trace;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

extern crate alloc;

mod chan;
mod reg;

pub use chan::{Channel, ChannelConfig};

use crate::reg::{DdmaRegister, DmaChannelRegisters};

/// DMA transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    /// Memory to Device (TX)
    MemoryToDevice,
    /// Device to Memory (RX)  
    DeviceToMemory,
}

/// DMA channel configuration
#[derive(Debug, Clone)]
pub struct DmaChannelConfig {
    /// Channel number (0-7)
    pub channel: u8,
    /// Peripheral slave ID (0-31)
    pub peripheral_id: u8,
    /// Transfer direction
    pub direction: DmaDirection,
    /// Enable timeout
    pub timeout_enable: bool,
    /// Timeout count
    pub timeout_count: u32,
}

/// DMA transfer descriptor
#[derive(Debug, Clone)]
pub struct DmaTransfer {
    /// Source address (memory address for TX, device address for RX)
    pub src_addr: u64,
    /// Destination address (device address for TX, memory address for RX)
    pub dst_addr: u32,
    /// Transfer size in bytes
    pub size: u32,
}

/// DDMA Controller
pub struct DDMA {
    reg: NonNull<reg::DdmaRegister>,
}

impl DDMA {
    /// Create a new DDMA instance
    pub fn new(base_addr: NonNull<u8>) -> Self {
        Self {
            reg: base_addr.cast(),
        }
    }

    fn reg(&self) -> &reg::DdmaRegister {
        unsafe { self.reg.as_ref() }
    }

    /// Initialize the DMA controller
    pub fn reset(&mut self) {
        let reg = unsafe { self.reg.as_ref() };

        reg.dma_ctl.write(reg::DMA_CTL::DMA_ENABLE::CLEAR);
        reg.dma_ctl.write(reg::DMA_CTL::DMA_SRST::SET);
        reg.dma_ctl.write(reg::DMA_CTL::DMA_SRST::CLEAR);

        let reg = unsafe { self.reg.as_ref() };

        // Enable global interrupt
        reg.dma_mask_int.write(reg::DMA_MASK_INT::GLOBAL_EN::SET);
    }

    pub fn enable(&mut self) {
        let reg = unsafe { self.reg.as_mut() };
        reg.dma_ctl.modify(reg::DMA_CTL::DMA_ENABLE::SET);
    }

    pub fn new_channel(&mut self, n: u8, config: ChannelConfig) -> Option<Channel> {
        assert!(n <= 7, "Channel number must be between 0 and 7");
        assert!(
            config.slave_id <= 31,
            "Peripheral ID must be between 0 and 31"
        );
        let channel = n as usize;

        if self.reg().is_channel_bind(channel) {
            trace!("Channel {} is already in use", n);
            return None; // Channel already in use
        }

        // Configure channel selection and enable using the new helper methods

        self.reg()
            .set_channel_config(channel, config.slave_id as u32, true);

        self.reg().set_channel_interrupt_mask(channel, true);

        // Calculate the address offset for the specified channel
        let channel_offset =
            DdmaRegister::CHANNEL_BASE_OFFSET + (n as usize) * DdmaRegister::CHANNEL_REGISTER_SIZE;

        // Get the base address of the register structure
        let base_addr = self.reg.as_ptr() as usize;
        let channel_addr = base_addr + channel_offset;

        self.reg().set_channel_bind(channel, true);

        let reg = unsafe { NonNull::new_unchecked(channel_addr as *mut DmaChannelRegisters) };
        Channel::new(n, reg, config)
    }

    /// Check if transfer is complete for a channel
    pub fn is_transfer_complete(&self, channel: u8) -> bool {
        if channel > 7 {
            return false;
        }

        let reg = unsafe { self.reg.as_ref() };
        reg.is_channel_complete(channel as usize)
    }

    /// Clear transfer complete status for a channel
    pub fn clear_transfer_complete(&mut self, channel: u8) {
        if channel <= 7 {
            let reg = unsafe { self.reg.as_mut() };
            reg.clear_channel_complete(channel as usize);
        }
    }

    /// Set channel interrupt mask
    pub fn set_channel_interrupt_mask(&mut self, channel: u8, mask: bool) {
        if channel <= 7 {
            let reg = unsafe { self.reg.as_mut() };
            reg.set_channel_interrupt_mask(channel as usize, mask);
        }
    }

    /// Get interrupt handler
    pub fn irq_handler(&self) -> IrqHandler {
        IrqHandler { reg: self.reg }
    }
}

/// Interrupt handler for DDMA
pub struct IrqHandler {
    reg: NonNull<reg::DdmaRegister>,
}

unsafe impl Send for IrqHandler {}
unsafe impl Sync for IrqHandler {}

impl IrqHandler {
    /// Handle DMA interrupt
    pub fn handle_irq(&self) {
        let reg = unsafe { self.reg.as_ref() };
        let status = reg.dma_stat.get();

        // Clear all completed transfers
        if status != 0 {
            reg.dma_stat.set(status);
        }
    }
}

// Common peripheral slave IDs for DDMA
pub mod peripheral_ids {
    /// UART0 TX DMA request
    pub const UART0_TX: u8 = 2;
    /// UART0 RX DMA request  
    pub const UART0_RX: u8 = 15;
    /// UART1 TX DMA request
    pub const UART1_TX: u8 = 3;
    /// UART1 RX DMA request
    pub const UART1_RX: u8 = 16;
}
