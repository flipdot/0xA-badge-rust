#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;
use crate::hal::{prelude::*, stm32, delay::Delay};

use cortex_m_rt::entry;
use cortex_m::peripheral::Peripherals;

mod front_leds;
use front_leds::FrontLeds;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32::Peripherals::take().unwrap();
    let core_peripherals = Peripherals::take().unwrap();

    // configure the clock (rcc = 'reset clock control')
    let mut rcc = peripherals.RCC.configure().sysclk(8.mhz()).freeze(&mut peripherals.FLASH);
    let mut delay = Delay::new(core_peripherals.SYST, &rcc);

    let pa = peripherals.GPIOA.split(&mut rcc);
    let pb = peripherals.GPIOB.split(&mut rcc);

    let mut front_leds = FrontLeds::setup(pa);

    loop {
        delay.delay_ms(33u8);
        front_leds.next();
    }
}