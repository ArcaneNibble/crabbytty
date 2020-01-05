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

#[app(device = atsamd11, peripherals = true)]
const APP: () = {
    struct Resources {
        USB_PERIPH: atsamd11::USB,
    }

    #[init]
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

        // Turn on interrupts
        cx.device.USB.intenset.write(|w| {
            w.eorst().enable()
        });
        cx.device.USB.epintenset0.write(|w| {
            w.rxstp().enable()
                .trcpt1().enable()
                .trcpt0().enable()
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

    #[task(binds = USB, resources = [USB_PERIPH])]
    fn usb_isr(cx: usb_isr::Context) {
        // hprintln!("USB ISR!").unwrap();

        if cx.resources.USB_PERIPH.intflag.read().eorst().is_pending() {
            hprintln!("USB reset").unwrap();
        }

        // Acknowledge all the USB device interrupts
        cx.resources.USB_PERIPH.intflag.modify(|_, w| {w});

        if cx.resources.USB_PERIPH.epintsmry.read().epint0().is_pending() {
            // TODO: Handle all the endpoints

            if cx.resources.USB_PERIPH.epintflag0.read().rxstp().is_pending() {
                hprintln!("EP0 setup").unwrap();
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
