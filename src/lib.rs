#![no_std]
#![recursion_limit = "512"]

use core::ptr::NonNull;
use log::{debug, trace};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

extern crate alloc;

mod chan;
mod reg;

pub use chan::{Channel, ChannelConfig};

use crate::reg::{DMA_STAT, DdmaRegister, DmaChannelRegisters};

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
        let reg = unsafe { self.reg.as_mut() };

        // Disable DDMA controller first
        reg.dma_ctl.write(reg::DMA_CTL::DMA_ENABLE::CLEAR);

        // Disable global interrupt
        reg.dma_mask_int.write(reg::DMA_MASK_INT::GLOBAL_EN::CLEAR);

        // Reset all channels first
        for chan_id in 0..8 {
            if reg.is_channel_bind(chan_id) {
                reg.set_channel_bind(chan_id, false);
            }
            // Disable channel interrupt
            reg.set_channel_interrupt_mask(chan_id, true); // true means mask (disable)
            // Clear any pending interrupts
            reg.clear_channel_complete(chan_id);
            // Reset channel configuration
            reg.set_channel_config(chan_id, 0, false);
        }

        // Perform software reset
        reg.dma_ctl.write(reg::DMA_CTL::DMA_SRST::SET);
        reg.dma_ctl.write(reg::DMA_CTL::DMA_SRST::CLEAR);
        reg.dma_mask_int.set(u32::MAX);
    }

    pub fn enable(&mut self) {
        self.reg()
            .dma_mask_int
            .modify(reg::DMA_MASK_INT::GLOBAL_EN::CLEAR);
        self.reg().dma_ctl.modify(reg::DMA_CTL::DMA_ENABLE::SET);
    }

    pub fn disable(&mut self) {
        let reg = unsafe { self.reg.as_mut() };
        reg.dma_ctl.write(reg::DMA_CTL::DMA_ENABLE::CLEAR);
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

        // According to C reference: First stop DMA controller
        self.disable();

        // Calculate the address offset for the specified channel
        let channel_offset =
            DdmaRegister::CHANNEL_BASE_OFFSET + (n as usize) * DdmaRegister::CHANNEL_REGISTER_SIZE;

        // Get the base address of the register structure
        let base_addr = self.reg.as_ptr() as usize;
        let channel_addr = base_addr + channel_offset;

        let reg = unsafe { NonNull::new_unchecked(channel_addr as *mut DmaChannelRegisters) };

        // Create channel first to get the buffer
        let channel_result = Channel::new(n, reg, config.clone())?;

        // Configure channel selection and bind (following C reference sequence)
        self.reg()
            .set_channel_config(channel, config.slave_id as u32, true);
        self.reg().set_channel_bind(channel, true);

        if config.irq {
            self.reg().set_channel_interrupt_mask(channel, false);
        } else {
            self.reg().set_channel_interrupt_mask(channel, true);
        }

        Some(channel_result)
    }

    /// Check if transfer is complete for a channel
    pub fn is_transfer_complete(&self, channel: u8) -> bool {
        if channel > 7 {
            return false;
        }

        let reg = unsafe { self.reg.as_ref() };
        reg.is_channel_complete(channel as usize)
    }

    /// Check DMA controller and channel status for debugging
    pub fn debug_status(&self, _channel: u8) -> (u32, u32, u32, u32) {
        let reg = unsafe { self.reg.as_ref() };
        let dma_ctl = reg.dma_ctl.get();
        let dma_stat = reg.dma_stat.get();
        let bind_status = reg.dma_channel_bind.get();
        let mask_int = reg.dma_mask_int.get();

        debug!("DMA Controller Status:");
        debug!(
            "  DMA_CTL: 0x{:08x} (enabled: {})",
            dma_ctl,
            (dma_ctl & 1) != 0
        );
        debug!("  DMA_STAT: 0x{:08x}", dma_stat);
        debug!("  BIND_STATUS: 0x{:08x}", bind_status);
        debug!("  MASK_INT: 0x{:08x}", mask_int);

        (dma_ctl, dma_stat, bind_status, mask_int)
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

#[derive(Debug, Clone, Copy, Default)]
pub struct CompletedChannels {
    channels: u8, // Bitmask of completed channels
}

impl CompletedChannels {
    /// Check if a specific channel has completed
    pub fn is_channel_completed(&self, channel: u8) -> bool {
        if channel < 8 {
            (self.channels & (1 << channel)) != 0
        } else {
            false
        }
    }

    /// Get a bitmask of all completed channels
    pub fn bitmask(&self) -> u8 {
        self.channels
    }

    fn set_channel_completed(&mut self, channel: u8) {
        if channel < 8 {
            self.channels |= 1 << channel;
        }
    }
}

impl IrqHandler {
    /// Handle DMA interrupt
    pub fn handle_irq(&self) -> CompletedChannels {
        let reg = unsafe { self.reg.as_ref() };
        let status = reg.dma_stat.extract();

        let mut completed = CompletedChannels::default();
        if status.is_set(DMA_STAT::CHAL0_SEL) {
            completed.set_channel_completed(0);
        }
        if status.is_set(DMA_STAT::CHAL1_SEL) {
            completed.set_channel_completed(1);
        }
        if status.is_set(DMA_STAT::CHAL2_SEL) {
            completed.set_channel_completed(2);
        }
        if status.is_set(DMA_STAT::CHAL3_SEL) {
            completed.set_channel_completed(3);
        }
        if status.is_set(DMA_STAT::CHAL4_SEL) {
            completed.set_channel_completed(4);
        }
        if status.is_set(DMA_STAT::CHAL5_SEL) {
            completed.set_channel_completed(5);
        }
        if status.is_set(DMA_STAT::CHAL6_SEL) {
            completed.set_channel_completed(6);
        }
        if status.is_set(DMA_STAT::CHAL7_SEL) {
            completed.set_channel_completed(7);
        }
        // Clear all completed transfers
        reg.dma_stat.set(u32::MAX);

        completed
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
