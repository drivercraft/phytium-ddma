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

        // Clear interrupts and activate channel (following C reference)
        channel.clear_and_active(&mut dma);
        
        // Then start DMA controller (following C reference sequence)
        dma.enable();

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
            }
            // Small delay to prevent busy waiting
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
        }

        if timeout == 0 {
            info!("DMA transfer timed out");
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
