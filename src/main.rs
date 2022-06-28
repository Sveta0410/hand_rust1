#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

//    let mut led = pins.d13.into_output();
    let mut pin_9 = pins.d9.into_output();

    let tc1 = dp.TC1;
    tc1.icr1.write(|w| unsafe { w.bits(4999) });
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
    loop{
        led.toggle();
        arduino_hal::delay_ms(1000);
//        analogWrite(pin_5, 100);
//        arduino_hal::delay_ms(1000);
//        analogWrite(pin_5, 200);
//        arduino_hal::delay_ms(1000);


        //pwm.set_channel_on_off(LEFT, 0, servo_min).unwrap();
    }
}
