use avr_hal_generic::hal::digital::v2::OutputPin;

pub struct ServoMotor{
    // d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,

    tc1: avr_device::atmega328p::TC1,
    // pin:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
    // angle: u32,
}

impl ServoMotor{
    pub fn write_5(mut my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PD5>, angle: u32)
        -> &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PD5>{
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        my_pin
    }
    pub fn write_10(mut my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB2>, angle: u32)
        -> &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB2>{
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
        my_pin
    }

    // pub fn new(
    //     d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
    //     tc1: avr_device::atmega328p::TC1,
    // ) -> Self {
    //     d9.into_output(); //pb1/d9/oc1a
    //     tc1.icr1.write(|w| unsafe { w.bits(4999) });
    //     tc1.tccr1a
    //     .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    //     tc1.tccr1b
    //     .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
    // Self{tc1}
    // }
    //
    // pub fn write_angle(tc1: avr_device::atmega328p::TC1, angle:u16) -> u16 {
    //     for duty in 100..=700 {
    //         tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
    //         arduino_hal::delay_ms(20);
    //     }
    //     let duty = 10;
    //     // self.tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
    //     duty
    // }
    // штука ниже похожа на рабочую, но не факт
    // pub fn write(&self, angle:u32) -> u32 {
    //     for _i in 0..50{
    //         let pulse_width = (angle * 11) + 500;
    //         // self.d9.into_output().set_high();
    //         arduino_hal::delay_us(pulse_width);
    //         // self.set_low();
    //         arduino_hal::delay_us(20000 - pulse_width);
    //     }
    //     angle
    // }
    // pub fn new(
    //     d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
    //     tc1: avr_device::atmega328p::TC1,
    // ) -> Self {
    //     d9.into_output(); //pb1/d9/oc1a
    //
    //     // setup TC1 so that it has a 50Hz signal.
    //     // oc1a is connected to the control wire of the servo
    //     // 50Hz is achieved by dividing the 16Mhz clock source
    //     // of the arduino by the 64 prescaler and then again by 5000 ( which is the TOP value set via icr1 )
    //     tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1a().match_clear().com1b().match_clear());
    //     tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
    //     tc1.icr1.write(|w| unsafe { w.bits(4999) });
    //
    //     ServoMotor{tc1}
    // }
    // pub fn write_angle(
    //     d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
    //     // d9:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB1>,
    //     angle:u32,
    // ) -> u32 {
    //     let mut our_pin = d9.into_output();
    //
    //     for _i in 0..50{
    //         let pulse_width = (angle * 11) + 500;
    //         our_pin.set_high();
    //         arduino_hal::delay_us(pulse_width);
    //         our_pin.set_low();
    //         arduino_hal::delay_us(20000 - pulse_width);
    //     }
    //     angle
    // }
    // pub fn write(&self, angle:u32) -> u32 {
    //     for _i in 0..50{
    //         let pulse_width = (angle * 11) + 500;
    //         self.set_high();
    //         arduino_hal::delay_us(pulse_width);
    //         self.set_low();
    //         arduino_hal::delay_us(20000 - pulse_width);
    //     }
    //     angle
    // }
}
    // pub fn write_angle(&self, angle:u16) -> u16 {
//         let duty = angle;
//         self.tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
//         // for _i in 0..50{
//         //     let pulse_width = (val * 11) + 500;
//         //     self.set_high().expect("TODO: panic message");
//         //     arduino_hal::delay_us(pulse_width);
//         //     self.set_low().expect("TODO: panic message");
//         //     arduino_hal::delay_us(20000 - pulse_width);
//         // }
//         duty
//     }
//     pub fn new(
//         mut pin:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
//         // avr_hal_generic::port::Pin<arduino_hal::port::mode::Input<arduino_hal::port::mode::Floating>, arduino_hal::atmega_hal::port::PB1>
//         // avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>
//         angle: u32,
//         tc1: avr_device::atmega328p::TC1,
//     ) -> Self {
//         pin.into_output();
//         Self {
//             tc1,
//         }
//     }
// }

//
// let servo1 = ServoMotor::write(pin, angle)?


// val = 180; // angle
// for _i in 0..50{
//     let pulse_width = (val * 11) + 500;
//     servo_pin.set_high();
//     arduino_hal::delay_us(pulse_width);
//     servo_pin.set_low();
//     arduino_hal::delay_us(20000 - pulse_width);
// }

//
