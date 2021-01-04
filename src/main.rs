#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::hal::pwm;
use arduino_uno::hal::pwm::Timer1Pwm;
use arduino_uno::prelude::*;

const STEP_SLEEP_MS: u16 = 10;
const ON_SLEEP_MS: u16 = 250;
const OFF_SLEEP_MS: u16 = 100;

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    // create a PWM timer
    let mut timer = Timer1Pwm::new(peripherals.TC1, pwm::Prescaler::Prescale64);
    // get pin D9 in PWM output mode
    let mut led = pins.d9.into_output(&mut pins.ddr).into_pwm(&mut timer);

    // enable PWM cycle
    led.enable();

    let duty_range = 0..=led.get_max_duty();
    loop {
        for i in duty_range.clone() {
            led.set_duty(i);
            arduino_uno::delay_ms(STEP_SLEEP_MS);
        }
        arduino_uno::delay_ms(ON_SLEEP_MS);

        for i in duty_range.clone().rev() {
            led.set_duty(i);
            arduino_uno::delay_ms(STEP_SLEEP_MS);
        }
        arduino_uno::delay_ms(OFF_SLEEP_MS);
    }
}
