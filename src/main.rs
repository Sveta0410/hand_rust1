#![no_std]
#![no_main]

// extern crate arduino_hal;
// extern crate avr_hal_generic;
//

// // use arduino_hal::delay_ms; // to write just delay_ms(100);
mod servo;
// use arduino_hal::prelude::*;
use servo::ServoMotor;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let servo = ServoMotor::new(pins.d9, dp.TC1);
    let mut servo_pin = pins.d10.into_output();
    let mut servo_pin1 = pins.d5.into_output();

    let mut led = pins.d13.into_output();

    let mut val;
    loop {
        // let duty = servo_pin.write_angle(10);
        val = 180; // angle
        led.toggle();
        arduino_hal::delay_ms(2000);
        for _i in 0..50{
            let pulse_width = (val * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        val = 0;
        arduino_hal::delay_ms(2000);

        for _i in 0..50 {
            let pulse_width = (val * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        arduino_hal::delay_ms(2000);

        val = 180; // angle
        led.toggle();
        for _i in 0..50{
            let pulse_width = (val * 11) + 500;
            servo_pin1.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin1.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        val = 0;
        arduino_hal::delay_ms(2000);

        for _i in 0..50{
            let pulse_width = (val * 11) + 500;
            servo_pin1.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin1.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        arduino_hal::delay_ms(2000);

    }
}