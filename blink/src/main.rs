#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;
use crate::hal::{prelude::*, stm32, delay::Delay};

use cortex_m_rt::entry;
use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32::Peripherals::take().unwrap();
    let core_peripherals = Peripherals::take().unwrap();

    // configure the clock (rcc = 'reset clock control')
    let mut rcc = peripherals.RCC.configure().sysclk(8.mhz()).freeze(&mut peripherals.FLASH);
    let mut delay = Delay::new(core_peripherals.SYST, &rcc);

    let gpioa = peripherals.GPIOA.split(&mut rcc);
    let gpiob = peripherals.GPIOB.split(&mut rcc);

    let mut led = cortex_m::interrupt::free(|critical_section| {
        gpiob.pb1.into_push_pull_output(critical_section)
    });

    loop {
        delay.delay_ms(1000u16);
        led.toggle();
    }
}