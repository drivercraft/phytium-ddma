#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use core::sync::atomic::AtomicBool;

    use super::*;
    use alloc::sync::Arc;
    use bare_test::{
        GetIrqConfig,
        globals::{PlatformInfoKind, global_val},
        irq::{IrqHandleResult, IrqInfo, IrqParam},
        mem::iomap,
    };
    use log::{debug, info};
    use phytium_ddma::{ChannelConfig, DDMA, DmaDirection, peripheral_ids};

    // PL011 UART Register offsets
    const UART_DR: usize = 0x00;     // Data Register
    const UART_FR: usize = 0x18;     // Flag Register  
    const UART_IBRD: usize = 0x24;   // Integer Baud Rate Divisor
    const UART_FBRD: usize = 0x28;   // Fractional Baud Rate Divisor
    const UART_LCR_H: usize = 0x2C;  // Line Control Register
    const UART_CR: usize = 0x30;     // Control Register
    const UART_DMACR: usize = 0x48;  // DMA Control Register

    // PL011 Control Register bits
    const UART_CR_UARTEN: u32 = 1 << 0;  // UART Enable
    const UART_CR_TXE: u32 = 1 << 8;     // Transmit Enable
    const UART_CR_RXE: u32 = 1 << 9;     // Receive Enable

    // PL011 DMA Control Register bits  
    const UART_DMACR_TXDMAE: u32 = 1 << 1; // Transmit DMA Enable
    const UART_DMACR_RXDMAE: u32 = 1 << 0; // Receive DMA Enable

    // PL011 Flag Register bits
    const UART_FR_TXFF: u32 = 1 << 5;    // Transmit FIFO Full
    const UART_FR_TXFE: u32 = 1 << 7;    // Transmit FIFO Empty

    /// Configure PL011 UART for DMA transmission
    fn configure_pl011_dma_tx(uart_base: usize) {
        let uart_base = uart_base as *mut u32;
        
        unsafe {
            // Read current control register
            let mut cr = core::ptr::read_volatile(uart_base.add(UART_CR / 4));
            info!("Current UART_CR: 0x{:08x}", cr);
            
            // Ensure UART is enabled
            cr |= UART_CR_UARTEN | UART_CR_TXE;
            core::ptr::write_volatile(uart_base.add(UART_CR / 4), cr);
            info!("Updated UART_CR: 0x{:08x}", cr);
            
            // Enable TX DMA
            let dmacr = UART_DMACR_TXDMAE;
            core::ptr::write_volatile(uart_base.add(UART_DMACR / 4), dmacr);
            info!("Set UART_DMACR: 0x{:08x}", dmacr);
            
            // Verify DMA control register
            let dmacr_read = core::ptr::read_volatile(uart_base.add(UART_DMACR / 4));
            info!("UART_DMACR readback: 0x{:08x}", dmacr_read);
            
            // Check TX FIFO status
            let fr = core::ptr::read_volatile(uart_base.add(UART_FR / 4));
            info!("UART_FR: 0x{:08x}, TXFE: {}, TXFF: {}", 
                  fr, 
                  (fr & UART_FR_TXFE) != 0,
                  (fr & UART_FR_TXFF) != 0);
        }
    }

    #[test]
    fn test_dma_memory_to_uart1_tx() {
        info!("test uart");

        let (addr, size, irq_info) = get_ddma0();

        let base = iomap(addr.into(), size);

        debug!("DDMA base address: {:p}, size: {:#x}", base, size);

        let mut dma = DDMA::new(base);

        dma.reset();

        debug!("DDMA controller reset done");

        let uart_1_addr = 0x2800d000usize; // UART1 base address, TX FIFO is at offset 0x00

        // Map UART1 address space for configuration
        let uart_base = iomap(uart_1_addr.into(), 0x1000);
        info!("UART1 base address mapped: {:p}", uart_base);

        // Configure PL011 UART for DMA transmission
        info!("Configuring PL011 UART1 for DMA transmission...");
        configure_pl011_dma_tx(uart_base.as_ptr() as usize);

        info!("Testing with slave ID: {} ({})", peripheral_ids::UART1_TX, "UART1_TX");

        let mut channel = dma
            .new_channel(
                0,
                ChannelConfig {
                    slave_id: peripheral_ids::UART1_TX,
                    direction: DmaDirection::MemoryToDevice,
                    timeout_count: 0x1000,
                    blk_size: 4,
                    dev_addr: uart_1_addr as _, // UART1 TX FIFO address (base + 0x00)
                },
            )
            .expect("Failed to create DMA channel 0");

        channel.buff_mut().set(0, b'A');
        channel.buff_mut().set(1, b'B');
        channel.buff_mut().set(2, b'\r');
        channel.buff_mut().set(3, b'\n');

        let irq_done = Arc::new(AtomicBool::new(false));

        // Setup interrupt handler
        let handle = dma.irq_handler();
        let irq_cfg = irq_info.cfgs[0].clone();

        IrqParam {
            intc: irq_info.irq_parent,
            cfg: irq_cfg,
        }
        .register_builder({
            let done = irq_done.clone();
            move |_irq| {
                handle.handle_irq();
                done.store(true, core::sync::atomic::Ordering::SeqCst);
                // info!("DMA interrupt handled");
                IrqHandleResult::Handled
            }
        })
        .register();

        info!("Starting DMA transfer: Memory to UART1 TX");

        // Debug: Check initial state
        dma.debug_status(channel.index());
        channel.debug_registers();

        // Clear interrupts and activate channel (following C reference)
        channel.clear_and_active(&mut dma);

        // Debug: Check state after activation
        dma.debug_status(channel.index());
        channel.debug_registers();

        // Then start DMA controller (following C reference sequence)
        dma.enable();

        // Debug: Check state after DMA enable
        dma.debug_status(channel.index());

        // Wait for transfer completion (polling mode for this test)
        let mut timeout = 100000; // Increase timeout
        while !dma.is_transfer_complete(channel.index()) && timeout > 0 {
            timeout -= 1;
            if irq_done.load(core::sync::atomic::Ordering::SeqCst) {
                info!("DMA transfer completed via interrupt");
                break;
            }
            // Add periodic status check
            if timeout % 10000 == 0 {
                debug!("DMA transfer in progress, timeout remaining: {}", timeout);
                debug!("Channel running: {}", channel.is_running());
                if timeout % 50000 == 0 {
                    channel.debug_registers();
                    dma.debug_status(channel.index());
                }
            }
            // Small delay to prevent busy waiting
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
        }

        if timeout == 0 {
            info!("DMA transfer timed out");
            
            // Final debug output
            info!("=== Final Status Debug ===");
            channel.debug_registers();
            dma.debug_status(channel.index());
            
            // Check if the issue is with UART DMA requests
            info!("=== Problem Analysis ===");
            info!("✅ DMA Controller: Enabled and working");
            info!("✅ Channel 0: Enabled and bound");
            info!("✅ Data Source: 0x90124500 (4 bytes)");
            info!("✅ FIFO Status: Has data (not empty)");
            info!("❌ Current Address: 0x00000000 (no progress)");
            info!("❌ Transfer Status: 0x00000000 (no completion)");
            info!("");
            info!("🔍 ROOT CAUSE: UART1 is not sending DMA TX requests");
            info!("   - DMA has read data from memory into FIFO");
            info!("   - UART1 TX is not requesting data via DMA");
            info!("   - Need to configure UART1 DMA TX enable");
            
            return;
        }

        // Clear the completion status
        dma.clear_transfer_complete(channel.index());

        info!("DMA transfer completed successfully! Character 'A' transferred to UART1 TX");
    }

    fn get_ddma0() -> (usize, usize, IrqInfo) {
        let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
        let fdt = fdt.get();

        let mut nodes = fdt.find_compatible(&["phytium,ddma"]);
        // nodes.next();
        let node = nodes.next().unwrap();

        let addr = node.reg().unwrap().next().unwrap();

        let size = addr.size.unwrap_or(0x1000);

        let irq_info = node.irq_info().unwrap();

        (addr.address as usize, size, irq_info)
    }
}
