#![no_std]
#![no_main]

use arduino_hal::prelude::_void_ResultVoidExt;
use panic_halt as _;

const WATER_SENSOR_ALERTING_THRESHOLD: u16 = 100;
const WATER_SENSOR_SAMPLING_RATE : u16 = 250;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());
    let water_sensor_pin = pins.a5.into_analog_input(&mut adc);

    let mut serial = arduino_hal::default_serial!(peripherals, pins, 9600);
    let mut water_sensor_rx;

    loop {
        water_sensor_rx = water_sensor_pin.analog_read(&mut adc);

        ufmt::uwriteln!(&mut serial, "{}", water_sensor_rx)
            .void_unwrap();

        if water_sensor_rx >= WATER_SENSOR_ALERTING_THRESHOLD {
            // TODO: Alerting logic goes here...
        }

        arduino_hal::delay_ms(WATER_SENSOR_SAMPLING_RATE);
    }
}
