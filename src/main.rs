#![no_std]
#![no_main]
use core::panic::PanicInfo;





#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[arduino_hal::entry]
fn main() -> ! {
    let mut val = 0;
    let mut _val_0 = 0;


    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut servo_pin = pins.d9.into_output();

    let mut led = pins.d13.into_output();
    loop {
        // servoPin.set_high(); // 0°
        // arduino_hal::delay_ms(1000);
        //
        // servoPin.set_low(); // 180°
        // arduino_hal::delay_ms(1000);
        val = val + 1;
        val = val * 180;
        led.toggle();
        for _i in 0..50{


            let pulse_width = (1 * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_ms(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_ms(20 - pulse_width / 1000);
        }
        val = 0;
        for _i in 0..50{
            let pulse_width = (0 * 11) + 500;
            servo_pin.set_high();
            arduino_hal::delay_ms(pulse_width);
            servo_pin.set_low();
            arduino_hal::delay_ms(20 - pulse_width / 1000);
        }
        // fn servoPulse(pin:u16, angle:u16){
        //     let pulseWidth = (angle * 11) + 500;
        //     servoPin.set_high();
        //     arduino_hal::delay_ms(pulseWidth);
        //     servoPin.set_low();
        //     arduino_hal::delay_ms(20 - pulseWidth / 1000);
        // }
    }
}
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//
//     // Important because this sets the bit in the DDR register!
//     pins.d9.into_output();
//
//     // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
//     // - Each count increases the duty-cycle by 4us.
//     // - Use OC1A which is connected to D9 of the Arduino Uno.
//     // let tc1 = dp.TC1;
//     // tc1.icr1.write(|w| unsafe { w.bits(4999) });
//     // tc1.tccr1a
//     //     .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
//     // tc1.tccr1b
//     //     .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
//     let mut led = pins.d13.into_output();
//     let mut pin = pins.d9.into_output().into_pwm(&mut timer);
//     loop {
//         led.toggle();
//         // 100 counts => 0.4ms
//         // 700 counts => 2.8ms
//         // for duty in 100..=700 {
//         //     tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
//         //     arduino_hal::delay_ms(20);
//         // }
//         pin.set_duty(0);
//         arduino_hal::delay_ms(1000);
//         pin.set_duty(10);
//         arduino_hal::delay_ms(1000);
//     }
// }

// #![no_std]
// #![no_main]
//
// use core::panic::PanicInfo;
// use rust_gpiozero::Servo;
//
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
//
// #[arduino_hal::entry]
// fn main() -> ! {
//
//
//
//     let mut myServo = Servo::new(9);
//
// //     // важная часть начало
// //     // let dp = arduino_hal::Peripherals::take().unwrap();
// //     // let pins = arduino_hal::pins!(dp);
// //     //
// //     // let mut led = pins.d13.into_output();
// //     // let mut pin_9 = pins.d9.into_output();
// // //     важная часть конец
// //
// //
// // //    let tc1 = dp.TC1;
// // //    tc1.icr1.write(|w| unsafe { w.bits(4999) });
// // //    tc1.tccr1a
// // //        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
// // //    tc1.tccr1b
// // //        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
//     loop{
//         myServo.min();
//         arduino_hal::delay_ms(1000);
//         myServo.mid();
//         arduino_hal::delay_ms(1000);
//         myServo.max();
//         arduino_hal::delay_ms(1000);
// //         // важная часть начало
// //         // led.toggle();
// //         // arduino_hal::delay_ms(1000);
// //         // конец
// //         // pin_9.set_high();
// //         // arduino_hal::delay_ms(1000);
// //         // pin_9.set_low();
// //
// //         // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
// //         // ufmt::uwrite!(serial, "\rpot_value: {}, angle: {}, duty: {}   ", pot_value, angle, duty).void_unwrap();
// // //        analogWrite(pin_5, 100);
// // //        arduino_hal::delay_ms(1000);
// // //        analogWrite(pin_5, 200);
// // //        arduino_hal::delay_ms(1000);
// //
// //
// //         //pwm.set_channel_on_off(LEFT, 0, servo_min).unwrap();
//     }
// }
