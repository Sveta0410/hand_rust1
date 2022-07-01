use avr_hal_generic::hal::digital::v2::OutputPin;

pub struct ServoMotor{
    tc1: avr_device::atmega328p::TC1,
    // pin:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
    // angle: u32,
}

impl ServoMotor{
    pub fn new(
        mut pin:avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>,
        // avr_hal_generic::port::Pin<arduino_hal::port::mode::Input<arduino_hal::port::mode::Floating>, arduino_hal::atmega_hal::port::PB1>
        // avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Input<avr_hal_generic::port::mode::Floating>, atmega_hal::port::PB1>
        angle: u32,
        tc1: avr_device::atmega328p::TC1,
    ) -> Self {
        pin.into_output();
        Self {
            tc1,
        }
    }
}

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
