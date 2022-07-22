pub struct ServoMotor{
    // pin: avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PD5>,
    // angle: u32,
}

impl ServoMotor{
    pub fn write_5(my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PD5>, angle: u32)
        {
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }

    }
    pub fn write_6(my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PD6>, angle: u32)
        {
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }

    }
    pub fn write_9(my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB1>, angle: u32)
        {
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }

    }
    pub fn write_10(my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB2>, angle: u32)
        {
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
    }
    pub fn write_11(my_pin:  &mut avr_hal_generic::port::Pin<avr_hal_generic::port::mode::Output, atmega_hal::port::PB3>, angle: u32)
        {
        for _i in 0..50{
            let pulse_width = (angle * 11) + 500;
            my_pin.set_high();
            arduino_hal::delay_us(pulse_width);
            my_pin.set_low();
            arduino_hal::delay_us(20000 - pulse_width);
        }
    }
}

