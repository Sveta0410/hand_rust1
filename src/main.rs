#![no_std]
#![no_main]

mod servo;
use servo::ServoMotor;

use core::panic::PanicInfo;
use avr_hal_generic::ufmt;
use embedded_hal::serial::Read;
use arduino_hal::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    ufmt::uwriteln!(&mut serial, "{}", 10).void_unwrap();

    let mut servo_pin = pins.d10.into_output();
    let _start = ServoMotor::write_10(&mut servo_pin, 0);

    let mut servo_pin1 = pins.d5.into_output();
    let _start = ServoMotor::write_5(&mut servo_pin1, 0);

    let mut led = pins.d13.into_output(); // delete later
    let b:u32 = nb::block!(serial.read()).unwrap().into();

    loop {
        led.toggle();
        let _work = ServoMotor::write_10(&mut servo_pin, b);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_10(&mut servo_pin, 0);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_5(&mut servo_pin1, 180);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_5(&mut servo_pin1, 0);
        arduino_hal::delay_ms(2000);
    }
}