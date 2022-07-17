#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use embedded_hal::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let mut start = false;
    // let start_sign = 36;
    let mut counter = 0;
    // let info_len = 5;
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();


    loop {
        // Read a byte from the serial connection
        let b:u8 = nb::block!(serial.read()).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();

        if b == 36{
            start = true;
            counter += 1;
            ufmt::uwriteln!(&mut serial, "start").void_unwrap();
        }
        while start {

            let b:u8 = nb::block!(serial.read()).void_unwrap();
            if counter == 1{
                ufmt::uwriteln!(&mut serial, "Got {} for the first finger!\r", b).void_unwrap();
                counter += 1;

            } else if counter == 2{
                ufmt::uwriteln!(&mut serial, "Got {} for the second finger!\r", b).void_unwrap();
                counter += 1;


            } else if counter == 3{
                ufmt::uwriteln!(&mut serial, "Got {} for 3 finger!\r", b).void_unwrap();
                counter += 1;

            } else if counter == 4{
                ufmt::uwriteln!(&mut serial, "Got {} for 4 finger!\r", b).void_unwrap();
                counter += 1;

            } else if counter == 5 {
                ufmt::uwriteln!(&mut serial, "Got {} for 5 finger!\r", b).void_unwrap();
                counter += 1;
                start = false;
            }

        }
        counter = 0;
        start = false;
        // Answer
        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}

// #![no_std]
// #![no_main]
//
// mod servo;
// use servo::ServoMotor;
//
// use core::panic::PanicInfo;
// use avr_hal_generic::usart::BaudrateArduinoExt;
// use avr_hal_generic::void::ResultVoidExt;
// use embedded_hal::serial::Read;
//
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
//
// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let mut pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
//
//     let mut servo_pin = pins.d10.into_output();
//     let _start = ServoMotor::write_10(&mut servo_pin, 0);
//
//     let mut servo_pin1 = pins.d5.into_output();
//     let _start = ServoMotor::write_5(&mut servo_pin1, 0);
//
//     let mut led = pins.d13.into_output(); // delete later
//     let b = 1;
//
//     ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
//
//     loop {
//         led.toggle();
//         arduino_hal::delay_ms(2000);
//         let b:u32 = nb::block!(serial.read()).void_unwrap().into();
//         // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
//
//         if b == 49{
//             let _work = ServoMotor::write_10(&mut servo_pin, 90);
//             arduino_hal::delay_ms(3000);
//             let _work = ServoMotor::write_10(&mut servo_pin, 180);
//             arduino_hal::delay_ms(2000);
//             let _work = ServoMotor::write_10(&mut servo_pin, 0);
//         } else{
//             // let _work = ServoMotor::write_10(&mut servo_pin, 180);
//         }
//         // let _work = ServoMotor::write_10(&mut servo_pin, 180);
//
//         // arduino_hal::delay_ms(2000);
//         // let _work = ServoMotor::write_10(&mut servo_pin, 0);
//         arduino_hal::delay_ms(2000);
//         let _work = ServoMotor::write_5(&mut servo_pin1, 180);
//         arduino_hal::delay_ms(2000);
//         let _work = ServoMotor::write_5(&mut servo_pin1, 0);
//         arduino_hal::delay_ms(2000);
//     }
// }