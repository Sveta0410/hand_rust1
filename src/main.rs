#![no_std]
#![no_main]

mod servo;
use servo::ServoMotor;

use core::panic::PanicInfo;
use avr_hal_generic::usart::BaudrateArduinoExt;
use avr_hal_generic::void::ResultVoidExt;
use embedded_hal::serial::Read;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut servo_pin = pins.d10.into_output();
    let _start = ServoMotor::write_10(&mut servo_pin, 0);

    let mut servo_pin1 = pins.d5.into_output();
    let _start = ServoMotor::write_5(&mut servo_pin1, 0);

    let mut led = pins.d13.into_output(); // delete later
    let b = 1;

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        let b:u32 = nb::block!(serial.read()).void_unwrap().into();
        led.toggle();
        if b % 10 == 1{
            let _work = ServoMotor::write_10(&mut servo_pin, 90);
            arduino_hal::delay_ms(3000);
            let _work = ServoMotor::write_10(&mut servo_pin, 180);
        } else{
            let _work = ServoMotor::write_10(&mut servo_pin, 180);
        }
        // let _work = ServoMotor::write_10(&mut servo_pin, 180);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_10(&mut servo_pin, 0);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_5(&mut servo_pin1, 180);
        arduino_hal::delay_ms(2000);
        let _work = ServoMotor::write_5(&mut servo_pin1, 0);
        arduino_hal::delay_ms(2000);
    }
}