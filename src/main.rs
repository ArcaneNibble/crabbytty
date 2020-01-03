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
        // Configure DFLL48M for 48 MHz and USB clock recovery mode
        // First read coarse calibration
        let nvm_cal_1 = unsafe { core::ptr::read(0x806024 as *const u32) };
        let nvm_coarse_cal = ((nvm_cal_1 >> (58 - 32)) & 0b111111) as u8;
        cx.device.SYSCTRL.dfllval.write(|w| {
            w.coarse().bits(nvm_coarse_cal).fine().bits(0x1ff)
        });
        // Set multiplier
        cx.device.SYSCTRL.dfllmul.write(|w| {
            w.cstep().bits(0x1f / 4).fstep().bits(10).mul().bits(48000)
        });
        // Set control register
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

        hprintln!("Hello world!").unwrap();
    }
};
