#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm::{self, delay}, peripheral, delay};
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use core::cell::{Cell, RefCell};
use stm32f4xx_hal::{
    pac::{self, stk::val},
    prelude::*,
    timer::{Channel1, Channel2},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Init");
    let dp = pac::Peripherals::take().unwrap();
    
    // Set up the system clock.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let channels = (Channel1::new(gpioa.pa8), Channel2::new(gpioa.pa9));

    let cp = cortex_m::peripheral::Peripherals::take();
    let mut delay: stm32f4xx_hal::timer::SysDelay = cp.unwrap().SYST.delay(&clocks);


    let pwm = dp.TIM1.pwm_hz(channels, 50.Hz(), &clocks).split();
    let (mut ch1, _ch2) = pwm;
    let max_duty = ch1.get_max_duty() as u32; 
    ch1.enable();

    // Zero signal
    rprintln!("Zero signal");
    ch1.set_duty(ch1.get_max_duty() / 20);

    rprintln!("Slight delay");
    delay.delay_ms(10000_u32);


    // Minimum signal is 1 ms => 5%
    // Max signal is 2 ms => 10%

    let min_limit = max_duty * 5 / 1000;
    let max_limit = max_duty / 100;

    rprintln!("Test");
    for value in 0..50 {
        //ch1.set_duty(map(value, 0, 100, min_limit, max_limit));
        let throttle = map(value, 0, 100, min_limit, max_limit);
        ch1.set_duty(throttle as u16);
        delay.delay_ms(12_u32);
    }

    loop {

        //rprintln!("{:?}", ch1.get_max_duty());

    }

        // Function that maps one range to another
    fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
        (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
    }


}
