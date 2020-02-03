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

const DEV_DESC: usb_justthebits::DeviceDescriptor = usb_justthebits::DeviceDescriptor {
    bLength: core::mem::size_of::<usb_justthebits::DeviceDescriptor>() as u8,
    bDescriptorType: usb_justthebits::DescriptorType::Device as u8,
    bcdUSB: 0x210,
    bDeviceClass: 0xFF,
    bDeviceSubClass: 0xFF,
    bDeviceProtocol: 0xFF,
    bMaxPacketSize0: 8,
    idVendor: 0xF055,
    idProduct: 0x0000,
    bcdDevice: 0,
    iManufacturer: 0,
    iProduct: 0,
    iSerialNumber: 0,
    bNumConfigurations: 1,
};

const CONF_DESC: usb_justthebits::ConfigurationDescriptor = usb_justthebits::ConfigurationDescriptor {
    bLength: core::mem::size_of::<usb_justthebits::ConfigurationDescriptor>() as u8,
    bDescriptorType: usb_justthebits::DescriptorType::Configuration as u8,
    wTotalLength: core::mem::size_of::<usb_justthebits::ConfigurationDescriptor>() as u16,
    bNumInterfaces: 0,
    bConfigurationValue: 1,
    iConfiguration: 0,
    bmAttributes: 0,
    bMaxPower: 250,
};

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        core::mem::size_of::<T>(),
    )
}

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
        // cx.resources.epdescs[0].bank1_pcksize.enable_auto_zlp();

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

        // hprintln!("Survived init!").unwrap();

        init::LateResources {
            USB_PERIPH: cx.device.USB,
        }
    }

    #[task(binds = USB, resources = [USB_PERIPH, epdescs, ep0outbuf, ep0inbuf])]
    fn usb_isr(cx: usb_isr::Context) {
        // hprintln!("USB ISR!").unwrap();
        static mut NEEDS_SET_ADDRESS: bool = false;
        static mut ADDRESS_TO_SET: u8 = 0;
        static mut IS_GETTING_DESCRIPTOR: bool = false;
        static mut GET_DESCRIPTOR_TYPE: u8 = 0;
        static mut GET_DESCRIPTOR_IDX: u8 = 0;
        static mut GET_DESCRIPTOR_LANG: u16 = 0;
        static mut GET_DESCRIPTOR_LEN: u16 = 0;
        static mut GET_DESCRIPTOR_OFFSOFAR: u16 = 0;
        static mut CURRENT_CONFIG: u8 = 0;

        if cx.resources.USB_PERIPH.intflag.read().eorst().is_pending() {
            // hprintln!("USB reset").unwrap();

            *NEEDS_SET_ADDRESS = false;
            *IS_GETTING_DESCRIPTOR = false;
            *CURRENT_CONFIG = 0;

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
                // hprintln!("EP0 setup").unwrap();

                let setuppkt: &usb_justthebits::SetupPacket = unsafe {
                    core::mem::transmute(&cx.resources.ep0outbuf.0)
                };

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
                                        if direction == 0 {
                                            let addr = setuppkt.wValue;
                                            // hprintln!("set address to {}", addr).unwrap();
                                            *NEEDS_SET_ADDRESS = true;
                                            *ADDRESS_TO_SET = addr as u8;
                                            handled = true;
                                        }
                                    },
                                    Ok(usb_justthebits::StandardRequest::GetDescriptor) => {
                                        if direction == 1 {
                                            let desctype = (setuppkt.wValue >> 8) as u8;
                                            let descidx = setuppkt.wValue as u8;
                                            let langidx = setuppkt.wIndex;
                                            let len = setuppkt.wLength;
                                            // hprintln!("get descriptor {} {} {} {}", desctype, descidx, langidx, len).unwrap();

                                            handled = true;

                                            *IS_GETTING_DESCRIPTOR = true;
                                            *GET_DESCRIPTOR_TYPE = desctype;
                                            *GET_DESCRIPTOR_IDX = descidx;
                                            *GET_DESCRIPTOR_LANG = langidx;
                                            *GET_DESCRIPTOR_LEN = len;
                                            *GET_DESCRIPTOR_OFFSOFAR = 0;

                                            // IN packets will come next
                                            match usb_justthebits::DescriptorType::try_from(*GET_DESCRIPTOR_TYPE) {
                                                Ok(usb_justthebits::DescriptorType::Device) => {
                                                    let dev_desc_bytes: &[u8] = unsafe {any_as_u8_slice(&DEV_DESC)};
                                                    *GET_DESCRIPTOR_LEN = core::cmp::min(*GET_DESCRIPTOR_LEN, dev_desc_bytes.len() as u16);

                                                    let bytesleft = *GET_DESCRIPTOR_LEN - *GET_DESCRIPTOR_OFFSOFAR;
                                                    let bytesnow = core::cmp::min(bytesleft, 8);

                                                    // hprintln!("descriptor at {} remain {} this {}", *GET_DESCRIPTOR_OFFSOFAR, bytesleft, bytesnow).unwrap();

                                                    cx.resources.epdescs[0].bank1_pcksize.set_byte_count(bytesnow as u32);
                                                    cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                                    cx.resources.ep0inbuf.0[..bytesnow as usize].copy_from_slice(&dev_desc_bytes[*GET_DESCRIPTOR_OFFSOFAR as usize..*GET_DESCRIPTOR_OFFSOFAR as usize + bytesnow as usize]);
                                                    cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                                        w.bk1rdy().set()
                                                    });

                                                    *GET_DESCRIPTOR_OFFSOFAR += bytesnow;
                                                },
                                                Ok(usb_justthebits::DescriptorType::Configuration) => {
                                                    let conf_desc_bytes: &[u8] = unsafe {any_as_u8_slice(&CONF_DESC)};
                                                    *GET_DESCRIPTOR_LEN = core::cmp::min(*GET_DESCRIPTOR_LEN, conf_desc_bytes.len() as u16);

                                                    let bytesleft = *GET_DESCRIPTOR_LEN - *GET_DESCRIPTOR_OFFSOFAR;
                                                    let bytesnow = core::cmp::min(bytesleft, 8);

                                                    // hprintln!("descriptor at {} remain {} this {}", *GET_DESCRIPTOR_OFFSOFAR, bytesleft, bytesnow).unwrap();

                                                    cx.resources.epdescs[0].bank1_pcksize.set_byte_count(bytesnow as u32);
                                                    cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                                    cx.resources.ep0inbuf.0[..bytesnow as usize].copy_from_slice(&conf_desc_bytes[*GET_DESCRIPTOR_OFFSOFAR as usize..*GET_DESCRIPTOR_OFFSOFAR as usize + bytesnow as usize]);
                                                    cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                                        w.bk1rdy().set()
                                                    });

                                                    *GET_DESCRIPTOR_OFFSOFAR += bytesnow;
                                                },
                                                _ => {
                                                    cx.resources.epdescs[0].bank1_pcksize.set_byte_count(0);
                                                    cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                                    cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                                        w.bk1rdy().set()
                                                    });
                                                }
                                            }
                                        }
                                    },
                                    Ok(usb_justthebits::StandardRequest::SetConfiguration) => {
                                        if direction == 0 {
                                            if setuppkt.wValue == 1 {
                                                // Only supported configuration is 1
                                                *CURRENT_CONFIG = 1;
                                                handled = true;
                                            }
                                        }
                                    },
                                    Ok(usb_justthebits::StandardRequest::GetConfiguration) => {
                                        if direction == 1 {
                                            handled = true;

                                            cx.resources.epdescs[0].bank1_pcksize.set_byte_count(1);
                                            cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                            cx.resources.ep0inbuf.0[0] = *CURRENT_CONFIG;
                                            cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                                w.bk1rdy().set()
                                            });
                                        }
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
                    // hprintln!("{:?}", setuppkt).unwrap();
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
                        cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                        cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                            w.bk1rdy().set()
                        });
                    } else {
                        // Eventually the request will be concluded with an OUT
                        cx.resources.USB_PERIPH.epstatusclr0.write(|w| {
                            w.bk0rdy().clear()
                        });
                    }
                }
            }
            if cx.resources.USB_PERIPH.epintflag0.read().trcpt1().is_pending() {
                // hprintln!("EP0 Bank1 IN done").unwrap();

                if *NEEDS_SET_ADDRESS {
                    cx.resources.USB_PERIPH.dadd.write(|w| {
                        w.adden().activate().dadd().bits(*ADDRESS_TO_SET)
                    });
                    *NEEDS_SET_ADDRESS = false;
                } else {
                    if *IS_GETTING_DESCRIPTOR {
                        // IN packets will come next
                        match usb_justthebits::DescriptorType::try_from(*GET_DESCRIPTOR_TYPE) {
                            Ok(usb_justthebits::DescriptorType::Device) => {
                                let dev_desc_bytes: &[u8] = unsafe {any_as_u8_slice(&DEV_DESC)};

                                let bytesleft = *GET_DESCRIPTOR_LEN - *GET_DESCRIPTOR_OFFSOFAR;
                                let bytesnow = core::cmp::min(bytesleft, 8);

                                // hprintln!("descriptor at {} remain {} this {}", *GET_DESCRIPTOR_OFFSOFAR, bytesleft, bytesnow).unwrap();

                                cx.resources.epdescs[0].bank1_pcksize.set_byte_count(bytesnow as u32);
                                cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                cx.resources.ep0inbuf.0[..bytesnow as usize].copy_from_slice(&dev_desc_bytes[*GET_DESCRIPTOR_OFFSOFAR as usize..*GET_DESCRIPTOR_OFFSOFAR as usize + bytesnow as usize]);
                                cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                    w.bk1rdy().set()
                                });

                                *GET_DESCRIPTOR_OFFSOFAR += bytesnow;

                                if *GET_DESCRIPTOR_OFFSOFAR == *GET_DESCRIPTOR_LEN {
                                    *IS_GETTING_DESCRIPTOR = false;
                                }
                            },
                            Ok(usb_justthebits::DescriptorType::Configuration) => {
                                let conf_desc_bytes: &[u8] = unsafe {any_as_u8_slice(&CONF_DESC)};

                                let bytesleft = *GET_DESCRIPTOR_LEN - *GET_DESCRIPTOR_OFFSOFAR;
                                let bytesnow = core::cmp::min(bytesleft, 8);

                                // hprintln!("descriptor at {} remain {} this {}", *GET_DESCRIPTOR_OFFSOFAR, bytesleft, bytesnow).unwrap();

                                cx.resources.epdescs[0].bank1_pcksize.set_byte_count(bytesnow as u32);
                                cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                cx.resources.ep0inbuf.0[..bytesnow as usize].copy_from_slice(&conf_desc_bytes[*GET_DESCRIPTOR_OFFSOFAR as usize..*GET_DESCRIPTOR_OFFSOFAR as usize + bytesnow as usize]);
                                cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                    w.bk1rdy().set()
                                });

                                *GET_DESCRIPTOR_OFFSOFAR += bytesnow;

                                if *GET_DESCRIPTOR_OFFSOFAR == *GET_DESCRIPTOR_LEN {
                                    *IS_GETTING_DESCRIPTOR = false;
                                }
                            },
                            _ => {
                                cx.resources.epdescs[0].bank1_pcksize.set_byte_count(0);
                                cx.resources.epdescs[0].bank1_pcksize.set_multi_packet_size(0);
                                cx.resources.USB_PERIPH.epstatusset0.write(|w| {
                                    w.bk1rdy().set()
                                });
                            }
                        }
                    }
                }
            }
            if cx.resources.USB_PERIPH.epintflag0.read().trcpt0().is_pending() {
                // hprintln!("EP0 Bank0 OUT done").unwrap();
            }

            // Acknowledge all the USB endpoint interrupts
            cx.resources.USB_PERIPH.epintflag0.modify(|_, w| {w});
        }
    }
};
