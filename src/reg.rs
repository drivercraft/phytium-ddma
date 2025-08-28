use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_bitfields! {
    u32,

    /// DMA Control Register (0x00)
    pub DMA_CTL [
        /// DMA Global Enable Control Signal
        /// 1: enable, 0: disable
        DMA_ENABLE OFFSET(0) NUMBITS(1) [],

        /// DMA Global Software Reset Control Signal
        /// 1: soft reset, 0: no soft reset
        DMA_SRST OFFSET(1) NUMBITS(1) [],

        /// Reserved bits
        RESERVED OFFSET(2) NUMBITS(30) []
    ],

    /// DMA Channel Configuration Register (0x04)
    pub DMA_CHAL_CONFIG [
        /// Channel 0 DMA request signal source selection (32 select 1)
        CHAL0_SEL OFFSET(0) NUMBITS(7) [],

        /// Channel 0 DMA request signal source selection enable flag
        CHAL0_SEL_EN OFFSET(7) NUMBITS(1) [],

        /// Channel 1 DMA request signal source selection (32 select 1)
        CHAL1_SEL OFFSET(8) NUMBITS(7) [],

        /// Channel 1 DMA request signal source selection enable flag
        CHAL1_SEL_EN OFFSET(15) NUMBITS(1) [],

        /// Channel 2 DMA request signal source selection (32 select 1)
        CHAL2_SEL OFFSET(16) NUMBITS(7) [],

        /// Channel 2 DMA request signal source selection enable flag
        CHAL2_SEL_EN OFFSET(23) NUMBITS(1) [],

        /// Channel 3 DMA request signal source selection (32 select 1)
        CHAL3_SEL OFFSET(24) NUMBITS(7) [],

        /// Channel 3 DMA request signal source selection enable flag
        CHAL3_SEL_EN OFFSET(31) NUMBITS(1) []
    ],

    /// DMA Channel Configuration Register 1 (0x28)
    pub DMA_CHAL_CONFIG1 [
        /// Channel 4 DMA request signal source selection (32 select 1)
        CHAL4_SEL OFFSET(0) NUMBITS(7) [],

        /// Channel 4 DMA request signal source selection enable flag
        CHAL4_SEL_EN OFFSET(7) NUMBITS(1) [],

        /// Channel 5 DMA request signal source selection (32 select 1)
        CHAL5_SEL OFFSET(8) NUMBITS(7) [],

        /// Channel 5 DMA request signal source selection enable flag
        CHAL5_SEL_EN OFFSET(15) NUMBITS(1) [],

        /// Channel 6 DMA request signal source selection (32 select 1)
        CHAL6_SEL OFFSET(16) NUMBITS(7) [],

        /// Channel 6 DMA request signal source selection enable flag
        CHAL6_SEL_EN OFFSET(23) NUMBITS(1) [],

        /// Channel 7 DMA request signal source selection (32 select 1)
        CHAL7_SEL OFFSET(24) NUMBITS(7) [],

        /// Channel 7 DMA request signal source selection enable flag
        CHAL7_SEL_EN OFFSET(31) NUMBITS(1) []
    ],

    /// DMA Status Register (0x08)
    pub DMA_STAT [
        /// Channel 0 DMA request block transfer complete
        /// Set to 1 when complete, software write 1 to clear
        CHAL0_SEL OFFSET(0) NUMBITS(1) [],

        /// Reserved
        RESERVED0 OFFSET(1) NUMBITS(3) [],

        /// Channel 1 DMA request block transfer complete
        CHAL1_SEL OFFSET(4) NUMBITS(1) [],

        /// Reserved
        RESERVED1 OFFSET(5) NUMBITS(3) [],

        /// Channel 2 DMA request block transfer complete
        CHAL2_SEL OFFSET(8) NUMBITS(1) [],

        /// Reserved
        RESERVED2 OFFSET(9) NUMBITS(3) [],

        /// Channel 3 DMA request block transfer complete
        CHAL3_SEL OFFSET(12) NUMBITS(1) [],

        /// Reserved
        RESERVED3 OFFSET(13) NUMBITS(3) [],

        /// Channel 4 DMA request block transfer complete
        CHAL4_SEL OFFSET(16) NUMBITS(1) [],

        /// Reserved
        RESERVED4 OFFSET(17) NUMBITS(3) [],

        /// Channel 5 DMA request block transfer complete
        CHAL5_SEL OFFSET(20) NUMBITS(1) [],

        /// Reserved
        RESERVED5 OFFSET(21) NUMBITS(3) [],

        /// Channel 6 DMA request block transfer complete
        CHAL6_SEL OFFSET(24) NUMBITS(1) [],

        /// Reserved
        RESERVED6 OFFSET(25) NUMBITS(3) [],

        /// Channel 7 DMA request block transfer complete
        CHAL7_SEL OFFSET(28) NUMBITS(1) [],

        /// Reserved
        RESERVED7 OFFSET(29) NUMBITS(3) []
    ],

    /// DMA Interrupt Mask Register (0x0C)
    pub DMA_MASK_INT [
        /// Channel 0 DMA request transfer complete interrupt output mask control
        CHAL0_MASK OFFSET(0) NUMBITS(1) [],

        /// Channel 1 DMA request transfer complete interrupt output mask control
        CHAL1_MASK OFFSET(1) NUMBITS(1) [],

        /// Channel 2 DMA request transfer complete interrupt output mask control
        CHAL2_MASK OFFSET(2) NUMBITS(1) [],

        /// Channel 3 DMA request transfer complete interrupt output mask control
        CHAL3_MASK OFFSET(3) NUMBITS(1) [],

        /// Channel 4 DMA request transfer complete interrupt output mask control
        CHAL4_MASK OFFSET(4) NUMBITS(1) [],

        /// Channel 5 DMA request transfer complete interrupt output mask control
        CHAL5_MASK OFFSET(5) NUMBITS(1) [],

        /// Channel 6 DMA request transfer complete interrupt output mask control
        CHAL6_MASK OFFSET(6) NUMBITS(1) [],

        /// Channel 7 DMA request transfer complete interrupt output mask control
        CHAL7_MASK OFFSET(7) NUMBITS(1) [],

        /// Reserved
        RESERVED OFFSET(8) NUMBITS(23) [],

        /// Global interrupt enable output control
        GLOBAL_EN OFFSET(31) NUMBITS(1) []
    ],

    /// DMA Channel Bind Register (0x20)
    pub DMA_CHANNEL_BIND [
        /// Status flag bits indicating whether channels 0-7 are bound to peripherals
        /// 1: Channel is bound to peripheral DMA request signal
        /// 0: Channel is not bound to peripheral DMA request signal
        DMA_CHANNEL_BIND OFFSET(0) NUMBITS(8) [],

        /// Reserved
        RESERVED OFFSET(8) NUMBITS(24) []
    ],

    /// DMA Global Capability Register (0x24) - Read Only
    pub DMA_GCAP [
        /// Read-only register indicating the total number of available effective channels in current DMA design
        DMA_GCAP OFFSET(0) NUMBITS(32) []
    ],

    /// Channel x Control Register (0x58+0x40*x)
    pub DMA_CHALX_CTL [
        /// Channel x enable control signal
        /// 1: Channel enabled, channel works
        /// 0: Channel disabled, channel doesn't work but registers and FIFO values remain unchanged
        CHALX_EN OFFSET(0) NUMBITS(1) [],

        /// Channel x reset control register
        /// When valid, channel related registers and FIFO are reset, 1 means valid
        /// Software must first disable the channel before resetting it
        CHALX_SRST OFFSET(1) NUMBITS(1) [],

        /// Channel x configured to receive which kind of request from peripheral
        /// 1: Receive peripheral dma_rx_req
        /// 0: Receive peripheral dma_tx_req
        /// Note: During normal operation, software is not allowed to operate this bit,
        /// only allowed when channel is disabled
        CHALX_MODE OFFSET(2) NUMBITS(1) [
            Tx = 0,
            Rx = 1
        ],

        /// Reserved
        RESERVED OFFSET(3) NUMBITS(29) []
    ],

    /// Channel x Status Register (0x5C+0x40*x)
    pub DMA_CHALX_STS [
        /// Channel x corresponding FIFO full status signal
        FIFO_FULL OFFSET(0) NUMBITS(1) [],

        /// Channel x corresponding FIFO empty status signal
        FIFO_EMPTY OFFSET(1) NUMBITS(1) [],

        /// Reserved
        RESERVED OFFSET(2) NUMBITS(30) []
    ],

    /// Channel x Timeout Count Register (0x60+0x40*x)
    pub DMA_CHALX_TIMEOUT_CNT [
        /// Timeout threshold time when timeout mechanism is enabled
        TIMEOUT_CNT OFFSET(0) NUMBITS(30) [],

        /// Reserved
        RESERVED OFFSET(30) NUMBITS(1) [],

        /// Timeout enable
        TIMEOUT_EN OFFSET(31) NUMBITS(1) []
    ]
}

register_structs! {
    /// Single DMA Channel Register Block Structure
    /// Each channel occupies 0x40 bytes (64 bytes) of address space
    pub DmaChannelRegisters {
        /// Channel DDR Upper Address Register
        (0x00 => pub ddr_upaddr: ReadWrite<u32>),
        /// Channel DDR Lower Address Register
        (0x04 => pub ddr_lwaddr: ReadWrite<u32>),
        /// Channel Device Address Register
        (0x08 => pub dev_addr: ReadWrite<u32>),
        /// Channel Transfer Size Register
        (0x0C => pub ts: ReadWrite<u32>),
        /// Channel Current Upper Address Register
        (0x10 => pub crt_upaddr: ReadWrite<u32>),
        /// Channel Current Lower Address Register
        (0x14 => pub crt_lwaddr: ReadWrite<u32>),
        /// Channel Control Register
        (0x18 => pub ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        /// Channel Status Register
        (0x1C => pub sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        /// Channel Timeout Count Register
        (0x20 => pub timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        /// Reserved space to next channel
        (0x24 => _reserved: [u8; 0x1C]),
        (0x40 => @END),
    }
}

register_structs! {
    /// DDMA Register Structure
    /// Base addresses: DMA0: 0x0002_8003_000, DMA1: 0x0002_8004_000
    pub DdmaRegister {
        /// Global Control Register
        (0x00 => pub dma_ctl: ReadWrite<u32, DMA_CTL::Register>),

        /// Channel Configuration Register (channels 0-3)
        (0x04 => pub dma_chal_config: ReadWrite<u32, DMA_CHAL_CONFIG::Register>),

        /// Interrupt Status Register
        (0x08 => pub dma_stat: ReadWrite<u32, DMA_STAT::Register>),

        /// Interrupt Mask Register
        (0x0C => pub dma_mask_int: ReadWrite<u32, DMA_MASK_INT::Register>),

        /// Upstream AXI Write Channel Configuration Register
        (0x10 => pub dma_upaxi_awconfig: ReadWrite<u32>),

        /// Upstream AXI Read Channel Configuration Register
        (0x14 => pub dma_upaxi_arconfig: ReadWrite<u32>),

        /// Downstream AXI Write Channel Configuration Register
        (0x18 => pub dma_dwnaxi_awconfig: ReadWrite<u32>),

        /// Downstream AXI Read Channel Configuration Register
        (0x1C => pub dma_dwnaxi_arconfig: ReadWrite<u32>),

        /// Channel Bind Register
        (0x20 => pub dma_channel_bind: ReadWrite<u32, DMA_CHANNEL_BIND::Register>),

        /// Global Capability Register (Read Only)
        (0x24 => pub dma_gcap: ReadOnly<u32, DMA_GCAP::Register>),

        /// Channel Configuration Register 1 (channels 4-7)
        (0x28 => pub dma_chal_config1: ReadWrite<u32, DMA_CHAL_CONFIG1::Register>),

        /// Reserved space before channel registers
        (0x2C => _reserved0: [u8; 0x14]),

        /// Channel registers block starting at 0x40
        /// Each channel uses 0x40 bytes, channels 0-7 supported
        (0x40 => _channel_registers: [u8; 0x1E4]),

        (0x224 => @END),
    }
}

impl DdmaRegister {
    /// Maximum number of supported DMA channels
    pub const MAX_CHANNELS: usize = 8;

    /// Size of each channel register block in bytes
    pub const CHANNEL_REGISTER_SIZE: usize = 0x40;

    /// Base offset for channel registers
    pub const CHANNEL_BASE_OFFSET: usize = 0x40;

    /// Configure channel selection for channels 0-7
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-7)
    /// * `sel` - Request signal source selection (0-127)
    /// * `enable` - Whether to enable the selection
    pub fn set_channel_config(&self, channel: usize, sel: u32, enable: bool) {
        match channel {
            0 => {
                self.dma_chal_config.modify(
                    DMA_CHAL_CONFIG::CHAL0_SEL.val(sel)
                        + DMA_CHAL_CONFIG::CHAL0_SEL_EN.val(enable as u32),
                );
            }
            1 => {
                self.dma_chal_config.modify(
                    DMA_CHAL_CONFIG::CHAL1_SEL.val(sel)
                        + DMA_CHAL_CONFIG::CHAL1_SEL_EN.val(enable as u32),
                );
            }
            2 => {
                self.dma_chal_config.modify(
                    DMA_CHAL_CONFIG::CHAL2_SEL.val(sel)
                        + DMA_CHAL_CONFIG::CHAL2_SEL_EN.val(enable as u32),
                );
            }
            3 => {
                self.dma_chal_config.modify(
                    DMA_CHAL_CONFIG::CHAL3_SEL.val(sel)
                        + DMA_CHAL_CONFIG::CHAL3_SEL_EN.val(enable as u32),
                );
            }
            4 => {
                self.dma_chal_config1.modify(
                    DMA_CHAL_CONFIG1::CHAL4_SEL.val(sel)
                        + DMA_CHAL_CONFIG1::CHAL4_SEL_EN.val(enable as u32),
                );
            }
            5 => {
                self.dma_chal_config1.modify(
                    DMA_CHAL_CONFIG1::CHAL5_SEL.val(sel)
                        + DMA_CHAL_CONFIG1::CHAL5_SEL_EN.val(enable as u32),
                );
            }
            6 => {
                self.dma_chal_config1.modify(
                    DMA_CHAL_CONFIG1::CHAL6_SEL.val(sel)
                        + DMA_CHAL_CONFIG1::CHAL6_SEL_EN.val(enable as u32),
                );
            }
            7 => {
                self.dma_chal_config1.modify(
                    DMA_CHAL_CONFIG1::CHAL7_SEL.val(sel)
                        + DMA_CHAL_CONFIG1::CHAL7_SEL_EN.val(enable as u32),
                );
            }
            _ => unreachable!(),
        }
    }

    /// Set interrupt mask for a specific channel
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-7)
    /// * `mask` - true to mask (disable) interrupt, false to unmask (enable)
    pub fn set_channel_interrupt_mask(&self, channel: usize, mask: bool) {
        match channel {
            0 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL0_MASK.val(mask as u32));
            }
            1 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL1_MASK.val(mask as u32));
            }
            2 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL2_MASK.val(mask as u32));
            }
            3 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL3_MASK.val(mask as u32));
            }
            4 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL4_MASK.val(mask as u32));
            }
            5 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL5_MASK.val(mask as u32));
            }
            6 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL6_MASK.val(mask as u32));
            }
            7 => {
                self.dma_mask_int
                    .modify(DMA_MASK_INT::CHAL7_MASK.val(mask as u32));
            }
            _ => {}
        }
    }

    /// Check if a channel transfer is complete
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-7)
    ///
    /// # Returns
    /// * `true` - if transfer is complete
    /// * `false` - if transfer is not complete or invalid channel
    pub fn is_channel_complete(&self, channel: usize) -> bool {
        match channel {
            0 => self.dma_stat.is_set(DMA_STAT::CHAL0_SEL),
            1 => self.dma_stat.is_set(DMA_STAT::CHAL1_SEL),
            2 => self.dma_stat.is_set(DMA_STAT::CHAL2_SEL),
            3 => self.dma_stat.is_set(DMA_STAT::CHAL3_SEL),
            4 => self.dma_stat.is_set(DMA_STAT::CHAL4_SEL),
            5 => self.dma_stat.is_set(DMA_STAT::CHAL5_SEL),
            6 => self.dma_stat.is_set(DMA_STAT::CHAL6_SEL),
            7 => self.dma_stat.is_set(DMA_STAT::CHAL7_SEL),
            _ => false,
        }
    }

    /// Clear channel transfer complete status
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-7)
    pub fn clear_channel_complete(&mut self, channel: usize) {
        match channel {
            0 => self.dma_stat.modify(DMA_STAT::CHAL0_SEL::SET),
            1 => self.dma_stat.modify(DMA_STAT::CHAL1_SEL::SET),
            2 => self.dma_stat.modify(DMA_STAT::CHAL2_SEL::SET),
            3 => self.dma_stat.modify(DMA_STAT::CHAL3_SEL::SET),
            4 => self.dma_stat.modify(DMA_STAT::CHAL4_SEL::SET),
            5 => self.dma_stat.modify(DMA_STAT::CHAL5_SEL::SET),
            6 => self.dma_stat.modify(DMA_STAT::CHAL6_SEL::SET),
            7 => self.dma_stat.modify(DMA_STAT::CHAL7_SEL::SET),
            _ => {}
        }
    }

    pub fn is_channel_bind(&self, channel: usize) -> bool {
        if channel >= Self::MAX_CHANNELS {
            return false;
        }
        (self.dma_channel_bind.get() & (1 << channel)) != 0
    }

    pub fn set_channel_bind(&self, channel: usize, bind: bool) {
        if channel >= Self::MAX_CHANNELS {
            return;
        }
        if bind {
            self.dma_channel_bind
                .set(self.dma_channel_bind.get() | (1 << channel));
        } else {
            self.dma_channel_bind
                .set(self.dma_channel_bind.get() & !(1 << channel));
        }
    }
}
