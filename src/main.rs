#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    // let mut val = 0;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut servo_pin = pins.d9.into_output();

    let mut led = pins.d13.into_output();
    loop {
        // val (angle)= 180;
        led.toggle();
        for _i in 0..50{
            let pulse_width = (180 * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20 - pulse_width / 1000);
        }
        // val (angle) = 0;
        arduino_hal::delay_ms(1000);
        for _i in 0..50{
            let pulse_width = 500;
            servo_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_us(20 - pulse_width / 1000);
        }
        arduino_hal::delay_ms(1000);

    }
}
