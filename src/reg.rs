use tock_registers::{
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
        CHALX_MODE OFFSET(2) NUMBITS(1) [],

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
    /// DDMA Register Structure
    /// Base addresses: DMA0: 0x0002_8003_000, DMA1: 0x0002_8004_000
    pub DdmaRegister {
        /// Global Control Register
        (0x00 => dma_ctl: ReadWrite<u32, DMA_CTL::Register>),

        /// Channel Configuration Register (channels 0-3)
        (0x04 => dma_chal_config: ReadWrite<u32, DMA_CHAL_CONFIG::Register>),

        /// Interrupt Status Register
        (0x08 => dma_stat: ReadWrite<u32, DMA_STAT::Register>),

        /// Interrupt Mask Register
        (0x0C => dma_mask_int: ReadWrite<u32, DMA_MASK_INT::Register>),

        /// Upstream AXI Write Channel Configuration Register
        (0x10 => dma_upaxi_awconfig: ReadWrite<u32>),

        /// Upstream AXI Read Channel Configuration Register
        (0x14 => dma_upaxi_arconfig: ReadWrite<u32>),

        /// Downstream AXI Write Channel Configuration Register
        (0x18 => dma_dwnaxi_awconfig: ReadWrite<u32>),

        /// Downstream AXI Read Channel Configuration Register
        (0x1C => dma_dwnaxi_arconfig: ReadWrite<u32>),

        /// Channel Bind Register
        (0x20 => dma_channel_bind: ReadWrite<u32, DMA_CHANNEL_BIND::Register>),

        /// Global Capability Register (Read Only)
        (0x24 => dma_gcap: ReadOnly<u32, DMA_GCAP::Register>),

        /// Channel Configuration Register 1 (channels 4-7)
        (0x28 => dma_chal_config1: ReadWrite<u32, DMA_CHAL_CONFIG1::Register>),

        /// Reserved space
        (0x2C => _reserved0: [u8; 0x14]),

        /// Channel 0 Registers
        (0x40 => dma_chal0_ddr_upaddr: ReadWrite<u32>),
        (0x44 => dma_chal0_ddr_lwaddr: ReadWrite<u32>),
        (0x48 => dma_chal0_dev_addr: ReadWrite<u32>),
        (0x4C => dma_chal0_ts: ReadWrite<u32>),
        (0x50 => dma_chal0_crt_upaddr: ReadWrite<u32>),
        (0x54 => dma_chal0_crt_lwaddr: ReadWrite<u32>),
        (0x58 => dma_chal0_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x5C => dma_chal0_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x60 => dma_chal0_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x64 => _reserved1: [u8; 0x1C]),

        /// Channel 1 Registers
        (0x80 => dma_chal1_ddr_upaddr: ReadWrite<u32>),
        (0x84 => dma_chal1_ddr_lwaddr: ReadWrite<u32>),
        (0x88 => dma_chal1_dev_addr: ReadWrite<u32>),
        (0x8C => dma_chal1_ts: ReadWrite<u32>),
        (0x90 => dma_chal1_crt_upaddr: ReadWrite<u32>),
        (0x94 => dma_chal1_crt_lwaddr: ReadWrite<u32>),
        (0x98 => dma_chal1_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x9C => dma_chal1_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0xA0 => dma_chal1_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0xA4 => _reserved2: [u8; 0x1C]),

        /// Channel 2 Registers
        (0xC0 => dma_chal2_ddr_upaddr: ReadWrite<u32>),
        (0xC4 => dma_chal2_ddr_lwaddr: ReadWrite<u32>),
        (0xC8 => dma_chal2_dev_addr: ReadWrite<u32>),
        (0xCC => dma_chal2_ts: ReadWrite<u32>),
        (0xD0 => dma_chal2_crt_upaddr: ReadWrite<u32>),
        (0xD4 => dma_chal2_crt_lwaddr: ReadWrite<u32>),
        (0xD8 => dma_chal2_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0xDC => dma_chal2_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0xE0 => dma_chal2_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0xE4 => _reserved3: [u8; 0x1C]),

        /// Channel 3 Registers
        (0x100 => dma_chal3_ddr_upaddr: ReadWrite<u32>),
        (0x104 => dma_chal3_ddr_lwaddr: ReadWrite<u32>),
        (0x108 => dma_chal3_dev_addr: ReadWrite<u32>),
        (0x10C => dma_chal3_ts: ReadWrite<u32>),
        (0x110 => dma_chal3_crt_upaddr: ReadWrite<u32>),
        (0x114 => dma_chal3_crt_lwaddr: ReadWrite<u32>),
        (0x118 => dma_chal3_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x11C => dma_chal3_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x120 => dma_chal3_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x124 => _reserved4: [u8; 0x1C]),

        /// Channel 4 Registers
        (0x140 => dma_chal4_ddr_upaddr: ReadWrite<u32>),
        (0x144 => dma_chal4_ddr_lwaddr: ReadWrite<u32>),
        (0x148 => dma_chal4_dev_addr: ReadWrite<u32>),
        (0x14C => dma_chal4_ts: ReadWrite<u32>),
        (0x150 => dma_chal4_crt_upaddr: ReadWrite<u32>),
        (0x154 => dma_chal4_crt_lwaddr: ReadWrite<u32>),
        (0x158 => dma_chal4_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x15C => dma_chal4_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x160 => dma_chal4_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x164 => _reserved5: [u8; 0x1C]),

        /// Channel 5 Registers
        (0x180 => dma_chal5_ddr_upaddr: ReadWrite<u32>),
        (0x184 => dma_chal5_ddr_lwaddr: ReadWrite<u32>),
        (0x188 => dma_chal5_dev_addr: ReadWrite<u32>),
        (0x18C => dma_chal5_ts: ReadWrite<u32>),
        (0x190 => dma_chal5_crt_upaddr: ReadWrite<u32>),
        (0x194 => dma_chal5_crt_lwaddr: ReadWrite<u32>),
        (0x198 => dma_chal5_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x19C => dma_chal5_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x1A0 => dma_chal5_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x1A4 => _reserved6: [u8; 0x1C]),

        /// Channel 6 Registers
        (0x1C0 => dma_chal6_ddr_upaddr: ReadWrite<u32>),
        (0x1C4 => dma_chal6_ddr_lwaddr: ReadWrite<u32>),
        (0x1C8 => dma_chal6_dev_addr: ReadWrite<u32>),
        (0x1CC => dma_chal6_ts: ReadWrite<u32>),
        (0x1D0 => dma_chal6_crt_upaddr: ReadWrite<u32>),
        (0x1D4 => dma_chal6_crt_lwaddr: ReadWrite<u32>),
        (0x1D8 => dma_chal6_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x1DC => dma_chal6_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x1E0 => dma_chal6_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x1E4 => _reserved7: [u8; 0x1C]),

        /// Channel 7 Registers
        (0x200 => dma_chal7_ddr_upaddr: ReadWrite<u32>),
        (0x204 => dma_chal7_ddr_lwaddr: ReadWrite<u32>),
        (0x208 => dma_chal7_dev_addr: ReadWrite<u32>),
        (0x20C => dma_chal7_ts: ReadWrite<u32>),
        (0x210 => dma_chal7_crt_upaddr: ReadWrite<u32>),
        (0x214 => dma_chal7_crt_lwaddr: ReadWrite<u32>),
        (0x218 => dma_chal7_ctl: ReadWrite<u32, DMA_CHALX_CTL::Register>),
        (0x21C => dma_chal7_sts: ReadWrite<u32, DMA_CHALX_STS::Register>),
        (0x220 => dma_chal7_timeout_cnt: ReadWrite<u32, DMA_CHALX_TIMEOUT_CNT::Register>),
        (0x224 => @END),
    }
}
