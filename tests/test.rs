#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use bare_test::{
        GetIrqConfig,
        globals::{PlatformInfoKind, global_val},
        irq::{IrqHandleResult, IrqInfo, IrqParam},
        mem::iomap,
    };
    use log::{debug, info};
    use phytium_ddma::DDMA;

    #[test]
    fn it_works() {
        let (addr, size, irq_info) = get_ddma1();

        let base = iomap(addr.into(), size);

        debug!("DDMA base address: {:p}, size: {:#x}", base, size);

        let mut dma = DDMA::new(base);

        let handle = dma.irq_handler();

        let irq_cfg = irq_info.cfgs[0].clone();

        IrqParam {
            intc: irq_info.irq_parent,
            cfg: irq_cfg,
        }
        .register_builder({
            move |_irq| {
                handle.handle_irq();
                IrqHandleResult::Handled
            }
        })
        .register();

        info!("DDMA test passed!");
    }

    fn get_ddma1() -> (usize, usize, IrqInfo) {
        let PlatformInfoKind::DeviceTree(fdt) = &global_val().platform_info;
        let fdt = fdt.get();

        let mut nodes = fdt.find_compatible(&["phytium,ddma"]);
        nodes.next();
        let node = nodes.next().unwrap();

        let addr = node.reg().unwrap().next().unwrap();

        let size = addr.size.unwrap_or(0x1000);

        let irq_info = node.irq_info().unwrap();

        (addr.address as usize, size, irq_info)
    }
}
