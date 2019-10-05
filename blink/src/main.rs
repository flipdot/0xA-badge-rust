#![no_main]
#![no_std]
// We're using a slightly outdated API, because the new one is not yet
// implemented for the board we are targeting.
#![allow(deprecated)]
#![feature(alloc_error_handler)]

use alloc::{boxed::Box, vec, vec::Vec};

use hal::{delay::Delay, prelude::*, stm32};
use stm32f0xx_hal as hal;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::digital::v1::ToggleableOutputPin;

fn boxed_toggleable_pin<T>(x: T) -> Box<dyn ToggleableOutputPin>
where
    T: ToggleableOutputPin + 'static,
{
    Box::new(x)
}

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

    let pb = peripherals.GPIOB.split(&mut rcc);

    let mut leds: Vec<Box<dyn ToggleableOutputPin>> =
        cortex_m::interrupt::free(|critical_section| {
            vec![
                boxed_toggleable_pin(pb.pb0.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb1.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb2.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb3.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb4.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb5.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb6.into_push_pull_output(critical_section)),
                boxed_toggleable_pin(pb.pb7.into_push_pull_output(critical_section)),
            ]
        });

    loop {
        delay.delay_ms(1000u32);

        for led in &mut leds {
            led.toggle();
        }
    }
}

extern crate alloc;

use alloc::alloc::Layout;
use alloc_cortex_m::CortexMHeap;
use panic_halt as _;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn handle_alloc_error(_: Layout) -> ! {
    panic!()
}
