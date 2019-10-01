use crate::hal::gpio::{gpioa::Parts, Output, Pin, PushPull};
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};

pub struct FrontLeds {
    output_pins: [Pin<Output<PushPull>>; 6],
}

impl FrontLeds {
    pub fn setup(pa: Parts) -> Self {
        let mut output_pins = cortex_m::interrupt::free(|critical_section| {
            [
                pa.pa7.into_push_pull_output(critical_section).downgrade(),
                pa.pa5.into_push_pull_output(critical_section).downgrade(),
                pa.pa4.into_push_pull_output(critical_section).downgrade(),
                pa.pa3.into_push_pull_output(critical_section).downgrade(),
                pa.pa2.into_push_pull_output(critical_section).downgrade(),
                pa.pa1.into_push_pull_output(critical_section).downgrade(),
            ]
        });


        output_pins[0].set_high().unwrap();

        Self { output_pins }
    }

    pub fn next(&mut self) -> &mut Self {
        for (i, pin) in self.output_pins.iter_mut().enumerate() {
            if pin.is_set_high().unwrap() {
                pin.set_low().unwrap();
                self.output_pins[(i + 1) % 6].set_high().unwrap();
                break;
            }
        }

        self
    }
}
