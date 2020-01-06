#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

extern crate usb_justthebits;

use cortex_m_semihosting::hprintln;
use rtfm::app;
use num_enum::TryFromPrimitive;
use core::convert::TryFrom;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[derive(TryFromPrimitive)]
pub enum USBCoreEPBufSize {
    _8 = 0,
    _16 = 1,
    _32 = 2,
    _64 = 3,
    _128 = 4,
    _256 = 5,
    _512 = 6,
    _1023 = 7,
}

#[repr(C)]
#[repr(packed)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct USBCoreEPDescriptorPcksize {
    pcksize: u32,
}

impl USBCoreEPDescriptorPcksize {
    pub fn is_auto_zlp(&self) -> bool {
        (self.pcksize & 0x80000000) != 0
    }

    pub fn disable_auto_zlp(&mut self) {
        self.pcksize &= !0x80000000;
    }

    pub fn enable_auto_zlp(&mut self) {
        self.pcksize |= 0x80000000;
    }

    pub fn get_byte_count(&self) -> u32 {
        self.pcksize & 0x3FFF
    }

    pub fn set_byte_count(&mut self, newval: u32) {
        let newval = newval & 0x3FFF;
        self.pcksize = (self.pcksize & !0x3FFF) | newval;
    }

    pub fn get_multi_packet_size(&self) -> u32 {
        (self.pcksize >> 14) & 0x3FFF
    }

    pub fn set_multi_packet_size(&mut self, newval: u32) {
        let newval = newval & 0x3FFF;
        self.pcksize = (self.pcksize & !0x0FFFC000) | (newval << 14);
    }

    pub fn get_size(&self) -> USBCoreEPBufSize {
        let val = ((self.pcksize >> 28) & 0b111) as u8;
        USBCoreEPBufSize::try_from(val).unwrap()
    }

    pub fn set_size(&mut self, newval: USBCoreEPBufSize) {
        self.pcksize = (self.pcksize & !0x70000000) | ((newval as u32) << 28);
    }
}

#[repr(C)]
#[repr(align(4))]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct USBCoreEPDescriptor {
    // FIXME: Addresses lie to the type system
    pub bank0_addr: u32,
    pub bank0_pcksize: USBCoreEPDescriptorPcksize,
    pub bank0_extreg: u16,
    pub bank0_statusbk: u8,
    _bank0_reserved: [u8; 5],
    pub bank1_addr: u32,
    pub bank1_pcksize: USBCoreEPDescriptorPcksize,
    _bank1_reserved1: u16,
    pub bank1_statusbk: u8,
    _bank1_reserved2: [u8; 5],
}

impl USBCoreEPDescriptor {
    pub const fn default() -> Self {
        Self {
            bank0_addr: 0,
            bank0_pcksize: USBCoreEPDescriptorPcksize{pcksize: 0},
            bank0_extreg: 0,
            bank0_statusbk: 0,
            _bank0_reserved: [0; 5],
            bank1_addr: 0,
            bank1_pcksize: USBCoreEPDescriptorPcksize{pcksize: 0},
            _bank1_reserved1: 0,
            bank1_statusbk: 0,
            _bank1_reserved2: [0; 5],
        }
    }
}

#[repr(align(4))]
pub struct Buf8Bytes([u8; 8]);

#[app(device = atsamd11, peripherals = true)]
const APP: () = {
    struct Resources {
        USB_PERIPH: atsamd11::USB,
        #[init(Buf8Bytes([0; 8]))]
        ep0outbuf: Buf8Bytes,
        #[init(Buf8Bytes([0; 8]))]
        ep0inbuf: Buf8Bytes,
        #[init([USBCoreEPDescriptor::default(); 1])]
        epdescs: [USBCoreEPDescriptor; 1],
    }

    #[init(resources = [epdescs, ep0outbuf, ep0inbuf])]
    fn init(cx: init::Context) -> init::LateResources {
        // Atmel has an "Errata 9905" that says that ondemand must be
        // disabled before doing any other writes. The ASF code seems to
        // set the control register this way.
        cx.device.SYSCTRL.dfllctrl.write(|w| {
            w.waitlock().beforelock()
                .bplckc().nobypass()
                .qldis().ql()
                .ccdis().cc()
                .ondemand().alwayson()
                .usbcrm().disabled()
                .llaw().keep()
                .stable().track()
                .mode().openloop()
                .enable().enabled()
        });
        // Wait for it to be ready
        while cx.device.SYSCTRL.pclksr.read().dfllrdy().is_notready() {}

        // Now we can actually set the multiplier and value
        // Configure DFLL48M for 48 MHz and USB clock recovery mode
        // Set multiplier
        cx.device.SYSCTRL.dfllmul.write(|w| {
            w.cstep().bits(0x1f / 4).fstep().bits(10).mul().bits(48000)
        });
        // Read coarse calibration and set value
        let nvm_cal_1 = unsafe { core::ptr::read(0x806024 as *const u32) };
        let nvm_coarse_cal = ((nvm_cal_1 >> (58 - 32)) & 0b111111) as u8;
        cx.device.SYSCTRL.dfllval.write(|w| {
            w.coarse().bits(nvm_coarse_cal).fine().bits(0x1ff)
        });

        // The ASF code seems to now set the control register to 0.
        cx.device.SYSCTRL.dfllctrl.write(|w| {
            w.waitlock().beforelock()
                .bplckc().nobypass()
                .qldis().ql()
                .ccdis().cc()
                .ondemand().alwayson()
                .usbcrm().disabled()
                .llaw().keep()
                .stable().track()
                .mode().openloop()
                .enable().disabled()
        });
        // Wait for it to be ready
        while cx.device.SYSCTRL.pclksr.read().dfllrdy().is_notready() {}

        // Set control register the way we actually wanted
        cx.device.SYSCTRL.dfllctrl.write(|w| {
            w.waitlock().beforelock()
                .bplckc().bypass()
                .qldis().ql()
                .ccdis().nocc()
                .ondemand().alwayson()
                .usbcrm().enabled()
                .llaw().keep()
                .stable().track()
                .mode().closedloop()
                .enable().enabled()
        });
        // Wait for it to be ready
        while cx.device.SYSCTRL.pclksr.read().dfllrdy().is_notready() {}

        // Need 1 wait state according to the datasheet table
        cx.device.NVMCTRL.ctrlb.write(|w| {
            w.rws().bits(1)
        });
        // Set CPU and bus dividers all to 1
        cx.device.PM.cpusel.write(|w| {
            w.cpudiv().div1()
        });
        cx.device.PM.apbasel.write(|w| {
            w.apbadiv().div1()
        });
        cx.device.PM.apbbsel.write(|w| {
            w.apbbdiv().div1()
        });
        cx.device.PM.apbcsel.write(|w| {
            w.apbcdiv().div1()
        });

        // Configure GCLK0 (which is also GCLK_MAIN) to use the DFLL
        cx.device.GCLK.gendiv.write(|w| {
            w.id().gclk0().div().bits(1)
        });
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk0()
                .src().dfll48m()
                .genen().enabled()
                .oe().notoutput()
                .divsel().div()
                .runstdby().running()
        });

        // Ok, we should be running at 48 MHz now

        // Turn everything else off to save power
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk1()
                .genen().disabled()
        });
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk2()
                .genen().disabled()
        });
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk3()
                .genen().disabled()
        });
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk4()
                .genen().disabled()
        });
        cx.device.GCLK.genctrl.write(|w| {
            w.id().gclk5()
                .genen().disabled()
        });

        // Turn the 8 MHz oscillator off to save power
        cx.device.SYSCTRL.osc8m.modify(|_, w| {
            w.enable().bit(false)
        });
        while cx.device.SYSCTRL.osc8m.read().enable().bit() {}

        // hprintln!("Hello world!").unwrap();

        // Set up clocks for USB
        cx.device.PM.ahbmask.modify(|_, w| {
            w.usb_().enabled()
        });
        cx.device.PM.apbbmask.modify(|_, w| {
            w.usb_().enabled()
        });
        cx.device.GCLK.clkctrl.write(|w| {
            w.id().usb()
                .gen().gclk0()
                .clken().enabled()
        });
        // Set up IOs for USB
        cx.device.PORT.pmux0_[12].write(|w| {
            w.pmuxe().g().pmuxo().g()
        });
        cx.device.PORT.pincfg0_[24].write(|w| {
            w.pmuxen().periph()
        });
        cx.device.PORT.pincfg0_[25].write(|w| {
            w.pmuxen().periph()
        });

        // Reset USB
        cx.device.USB.ctrla.write(|w| {
            w.swrst().reset()
        });
        while cx.device.USB.ctrla.read().swrst().is_reset() ||
            cx.device.USB.syncbusy.read().swrst().is_syncing() {}

        // Pad calibration
        let nvm_transn_cal = ((nvm_cal_1 >> (45 - 32)) & 0b11111) as u8;
        let nvm_transp_cal = ((nvm_cal_1 >> (50 - 32)) & 0b11111) as u8;
        let nvm_trim_cal = ((nvm_cal_1 >> (55 - 32)) & 0b111) as u8;
        cx.device.USB.padcal.write(|w| {
            w.trim().bits(nvm_trim_cal)
                .transn().bits(nvm_transn_cal)
                .transp().bits(nvm_transp_cal)
        });

        // Enable USB
        cx.device.USB.ctrla.write(|w| {
            w.enable().enabled().mode().device()
        });

        // Set up descriptors
        cx.resources.epdescs[0].bank0_addr =
            cx.resources.ep0outbuf.0.as_mut_ptr() as u32;
        cx.resources.epdescs[0].bank0_pcksize.set_size(USBCoreEPBufSize::_8);

        cx.resources.epdescs[0].bank1_addr =
            cx.resources.ep0inbuf.0.as_mut_ptr() as u32;
        cx.resources.epdescs[0].bank1_pcksize.set_size(USBCoreEPBufSize::_8);
        cx.resources.epdescs[0].bank1_pcksize.enable_auto_zlp();

        cx.device.USB.descadd.write(|w| {
            w.descadd().bits(cx.resources.epdescs.as_ptr() as u32)
        });

        // Turn on EP0 (which won't change during reset)
        cx.device.USB.epcfg0.write(|w| {
            w.eptype0().controlout()
                .eptype1().controlin()
        });

        // Turn on interrupt on reset
        cx.device.USB.intenset.write(|w| {
            w.eorst().enable()
        });

        // Attach USB
        cx.device.USB.ctrlb.write(|w| {
            w.spdconf().fs().detach().attach()
        });

        hprintln!("Survived init!").unwrap();

        init::LateResources {
            USB_PERIPH: cx.device.USB,
        }
    }

    #[task(binds = USB, resources = [USB_PERIPH, epdescs, ep0outbuf, ep0inbuf])]
    fn usb_isr(cx: usb_isr::Context) {
        // hprintln!("USB ISR!").unwrap();

        if cx.resources.USB_PERIPH.intflag.read().eorst().is_pending() {
            hprintln!("USB reset").unwrap();

            // TODO: Would have to set up all the endpoints here

            // Set up all the endpoint interrupts
            cx.resources.USB_PERIPH.epintenset0.write(|w| {
                w.rxstp().enable()
                    .trcpt1().enable()
                    .trcpt0().enable()
            });
        }

        // Acknowledge all the USB device interrupts
        cx.resources.USB_PERIPH.intflag.modify(|_, w| {w});

        if cx.resources.USB_PERIPH.epintsmry.read().epint0().is_pending() {
            // TODO: Handle all the endpoints

            if cx.resources.USB_PERIPH.epintflag0.read().rxstp().is_pending() {
                hprintln!("EP0 setup").unwrap();

                let setuppkt: &usb_justthebits::SetupPacket = unsafe {
                    core::mem::transmute(&cx.resources.ep0outbuf.0)
                };

                hprintln!("{:?}", setuppkt).unwrap();

                let direction = setuppkt.bmRequestType >> 7;
                let reqtype = (setuppkt.bmRequestType >> 5) & 0b11;
                let reqrec = setuppkt.bmRequestType & 0b11111;

                let mut handled = false;

                match usb_justthebits::RequestTypeType::try_from(reqtype) {
                    Ok(usb_justthebits::RequestTypeType::Standard) => {
                        // USB standard requests
                        match usb_justthebits::RequestTypeRecipient::try_from(reqrec) {
                            Ok(usb_justthebits::RequestTypeRecipient::Device) => {
                                // Device requests
                                match usb_justthebits::StandardRequest::try_from(setuppkt.bRequest) {
                                    Ok(usb_justthebits::StandardRequest::SetAddress) => {
                                        let addr = setuppkt.wValue;
                                        hprintln!("set address to {}", addr).unwrap();
                                        handled = true;
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }

                if !handled {
                    // Set stall for future requests
                    cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                        w.stallrq0().set().stallrq1().set()
                    });
                } else {
                    // Handled

                    // TODO: What if data is expected?

                    if direction == 0 {
                        // Host to device so set up for ZLP IN
                        cx.resources.epdescs[0].bank1_pcksize.set_byte_count(0);
                        cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                            w.bk1rdy().set()
                        });
                    }
                }
            }
            if cx.resources.USB_PERIPH.epintflag0.read().trcpt1().is_pending() {
                hprintln!("EP0 Bank1 IN done").unwrap();
            }
            if cx.resources.USB_PERIPH.epintflag0.read().trcpt0().is_pending() {
                hprintln!("EP0 Bank0 OUT done").unwrap();
            }

            // Acknowledge all the USB endpoint interrupts
            cx.resources.USB_PERIPH.epintflag0.modify(|_, w| {w});
        }
    }
};
