#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::ToString;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut servo_pin = pins.d9.into_output();
    let mut servo_pin1 = pins.d5.into_output();

    let mut led = pins.d13.into_output(); //delete later

    let mut val; //we'll delete it later

    let input_all_str = "10100".to_string(); // input from python
    let input_all = input_all_str.parse::<i32>().unwrap(); // from string to int

    loop {



        // for tests:
        val = 180; // angle
        led.toggle();
        for _i in 0..50{
            let pulse_width = (val * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width - 1000);
        }
        val = 0;
        arduino_hal::delay_ms(2000);
        for _i in (0..50).rev(){
            let pulse_width = (val * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width - 1000);
        }
        arduino_hal::delay_ms(2000);

        val = 180; // angle
        led.toggle();
        for _i in 0..50{
            let pulse_width = (val * 11) + 500;
            servo_pin1.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin1.set_low();
            arduino_hal::delay_us(20000 - pulse_width - 1000);
        }
        val = 0;
        arduino_hal::delay_ms(2000);
        for _i in (0..50).rev(){
            let pulse_width = (val * 11) + 500;
            servo_pin1.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin1.set_low();
            arduino_hal::delay_us(20000 - pulse_width - 1000);
        }
        arduino_hal::delay_ms(2000);
    }
}
