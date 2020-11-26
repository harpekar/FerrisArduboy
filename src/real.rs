#![no_std]
#![no_main]

//Pull in the panic handler from panic-halt

extern crate panic_halt;

use arduboy::prelude::*;
use panic_halt as _;

#[arduboy::entry]
fn main() -> ! {
    let dp = arduboy::Peripherals::take().unwrap();

    let mut pins = arduboy::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut led0 = pins.led0.into_output(&mut pins.ddr);
    let mut led1 = pins.led1.into_output(&mut pins.ddr);
    let mut led2 = pins.led2.into_output(&mut pins.ddr);

    let mut clk = pins.sck.into_output(&mut pins.ddr);
    let mut mosi = pins.mosi.into_output(&mut pins.ddr);
    let mut miso = pins.miso;
    let mut dc = pins.dc.into_output(&mut pins.ddr);

    let mut speaker = pins.speaker1.into_output(&mut pins.ddr);

    speaker.set_high().void_unwrap();
    led0.set_high().void_unwrap();
    led1.set_high().void_unwrap();
    led2.set_high().void_unwrap();

    // 4 Pin SPI Interface to screen

    


    let mut leds = [led0.downgrade(), led1.downgrade(), led2.downgrade()];

    loop {
        for i in 0..3 {
            
            speaker.toggle().void_unwrap(); 
            
            leds[i].toggle().void_unwrap();
            leds[(i + 2) % 3].toggle().void_unwrap();
            arduboy::delay_ms(200);
        }
    }
}
