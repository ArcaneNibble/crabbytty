#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_semihosting::hprintln;
use rtfm::app;

#[app(device = atsamd11, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
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

        hprintln!("Hello world!").unwrap();
    }
};
