//! Exposes a serial device over the USB interface that echoes back
//! bytes written to it.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::efc::*;
    use hal::fugit::RateExtU32;
    use hal::pmc::*;
    use hal::usb::usb_device::{bus::UsbBusAllocator, prelude::*};
    use rtt_target::{rprint, rprintln, rtt_init_print};
    use usbd_serial::{SerialPort, USB_CLASS_CDC};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, hal::usb::Usb>,
        serial: SerialPort<'static, hal::usb::Usb>,
        buf: [u8; 64],
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut USB_ALLOCATOR: Option<UsbBusAllocator<hal::usb::Usb>> = None;

        rtt_init_print!();
        rprint!("init...");

        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let mainck = pmc
            .get_mainck(MainCkSource::ExternalNormal(12.MHz()))
            .unwrap();
        let upllck = pmc.get_upllck(&mainck, ctx.device.UTMI).unwrap();
        let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
        pmc.get_hclk(
            HostClockConfig {
                pres: MckPrescaler::CLK_2,
                div: MckDivider::EQ_PCK,
            },
            &upllckdiv,
            &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
        )
        .unwrap();

        let usb_alloc = unsafe {
            USB_ALLOCATOR =
                Some(hal::usb::Usb::new(ctx.device.USBHS, &mut pmc, &upllck).into_usb_allocator());
            USB_ALLOCATOR.as_ref().unwrap()
        };

        let serial = SerialPort::new(&usb_alloc);
        let usb_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0xdead, 0xbeef))
            .manufacturer("ATSAMx7x HAL Contributors")
            .product("Serial port echo")
            .serial_number("N/A")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64) // makes control transfers 8x faster
            .build();

        rprintln!(" done");

        (
            Shared {},
            Local {
                serial,
                usb_dev,
                buf: [0; 64],
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBHS, local = [serial, usb_dev, buf])]
    fn usb(ctx: usb::Context) {
        let usb::LocalResources {
            serial,
            usb_dev,
            buf,
        } = ctx.local;

        if !usb_dev.poll(&mut [serial]) {
            return;
        }

        let count = match serial.read(buf) {
            Ok(count) => count,
            Err(UsbError::WouldBlock) => {
                // No data received
                return;
            }
            Err(e) => {
                rprintln!("USB read error: {:?}", e);
                return;
            }
        };

        let echo = &buf[..count];

        match serial.write(echo) {
            Ok(count) => {
                rprintln!("Echoed back {:x?} ({} bytes)", echo, count);
            }
            Err(e) => {
                rprintln!("USB write error: {:?}", e);
            }
        }
    }
}