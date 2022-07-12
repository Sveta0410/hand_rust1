#![no_std]
#![no_main]

mod servo;
use servo::ServoMotor;

use core::panic::PanicInfo;
use avr_hal_generic::usart::BaudrateArduinoExt;
use avr_hal_generic::void::ResultVoidExt;
use embedded_hal::serial::Read;
use ruduino::legacy::serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

//    !!! not working because i can't find crate arduino_uno  !!!

//     let mut serial = arduino_uno::Serial::new(
//         dp.USART0,
//         pins.d0,
//         pins.d1.into_output(),
//         57600.into_baudrate(),
// );


    // code below does not work because of this error:

//     error: proc macro panicked
//   --> /home/sveta/.cargo/registry/src/github.com-1ecc6299db9ec823/avr-config-2.0.1/src/cpu_frequency.rs:22:36
//    |
// 22 | const CPU_FREQUENCY_HZ_IMPL: u32 = value_from_env!("AVR_CPU_FREQUENCY_HZ": u32); // Must be set whenever AVR is being targeted.
//    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//    |
//    = help: message: no value available for environment variable 'AVR_CPU_FREQUENCY_HZ'
//
// error: aborting due to previous error
//
// error: could not compile `avr-config`

    //
    // serial::Serial::new(9600)
    //     .character_size(serial::CharacterSize::EightBits)
    //     .mode(serial::Mode::Asynchronous)
    //     .parity(serial::Parity::Disabled)
    //     .stop_bits(serial::StopBits::OneBit)
    //     .configure();
    //
    //
    // if let Some(b) = serial::try_receive() {
    // serial::transmit(b);
    // serial::transmit(b);
    // }

    let mut servo_pin = pins.d10.into_output();
    let _start = ServoMotor::write_10(&mut servo_pin, 0);

    let mut servo_pin1 = pins.d5.into_output();
    let _start = ServoMotor::write_5(&mut servo_pin1, 0);

    let mut led = pins.d13.into_output(); // delete later
    let b = 1;

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