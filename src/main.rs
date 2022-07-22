#![no_std]
#![no_main]

mod servo;
use servo::ServoMotor;

use core::panic::PanicInfo;

use arduino_hal::prelude::*;
use panic_halt as _;

use embedded_hal::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let mut start = false;
    let mut counter = 0;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let mut servo_pin = pins.d10.into_output();
    let _start = ServoMotor::write_10(&mut servo_pin, 0);

    let mut servo_pin1 = pins.d5.into_output();
    let _start = ServoMotor::write_5(&mut servo_pin1, 0);

    let mut led = pins.d13.into_output(); // delete later

    let mut b1=0;
    let mut b2=0;
    let mut b3=0;
    let mut b4=0;
    let mut b5= 0;
    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();
        if b == 36{
            start = true;
            counter = 1;
            // ufmt::uwriteln!(&mut serial, "start").void_unwrap();
        }
        while start {
            let b1 = nb::block!(serial.read()).void_unwrap();
        // if b1 == 48{
        //     let _work = ServoMotor::write_10(&mut servo_pin, 0);
        // } else if b1 == 49{
        //     let _work = ServoMotor::write_10(&mut servo_pin, 180);
        // }
            let b2 = nb::block!(serial.read()).void_unwrap();
            let b3 = nb::block!(serial.read()).void_unwrap();
            let b4 = nb::block!(serial.read()).void_unwrap();
            let b5 = nb::block!(serial.read()).void_unwrap();
            ufmt::uwriteln!(&mut serial, "Got {} {} {} {} {}!\r", b1, b2, b3, b4, b5).void_unwrap();
            start = false;

        }

    }
}
