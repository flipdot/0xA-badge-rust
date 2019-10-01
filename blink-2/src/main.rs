#![no_main]
#![no_std]
#![allow(deprecated)]

#[allow(unused)]
use panic_halt;

a
use hal::{delay::Delay, prelude::*, stm32};
use stm32f0xx_hal as hal;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32::Peripherals::take().unwrap();
    let core_peripherals = Peripherals::take().unwrap();

    // configure the clock (rcc = 'reset clock control')
    let mut rcc = peripherals
        .RCC
        .configure()
        .sysclk(8.mhz())
        .freeze(&mut peripherals.FLASH);
    let mut delay = Delay::new(core_peripherals.SYST, &rcc);

    let gpiob = peripherals.GPIOB.split(&mut rcc);

    let mut led = cortex_m::interrupt::free(|critical_section| {
        gpiob.pb1.into_push_pull_output(critical_section)
    });

    loop {
        delay.delay_ms(1000u16);
        led.toggle();
    }
}
