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

    let mut input_all_str = "10100".to_string(); // input from python
    let mut input_all = input_all_str.parse::<i32>().unwrap(); // from string to int
    let mut val_0 = input_all/10000; //for example thumb
    let mut val_1 = (input_all/1000)%10; //index
    let mut val_2 = (input_all/100)%10; // middle
    let mut val_3 = (input_all/10)%10; // ring
    let mut val_4 = input_all%10; //pinky
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
