#![no_std]
#![no_main]

mod servo;
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

    let mut servo_pin = pins.d10.into_output();
    let start = ServoMotor::write_10(&mut servo_pin, 0);

    let mut servo_pin1 = pins.d5.into_output();
    let start = ServoMotor::write_5(&mut servo_pin1, 0);

    let mut led = pins.d13.into_output(); // delete later

    let mut val = 180;

    loop {
        led.toggle();
        let work = ServoMotor::write_10(&mut servo_pin, 180);
        arduino_hal::delay_ms(2000);
        let work = ServoMotor::write_10(&mut servo_pin, 0);
        arduino_hal::delay_ms(2000);
        let work = ServoMotor::write_5(&mut servo_pin1, 180);
        arduino_hal::delay_ms(2000);
        let work = ServoMotor::write_5(&mut servo_pin1, 0);
        arduino_hal::delay_ms(2000);
    }
}