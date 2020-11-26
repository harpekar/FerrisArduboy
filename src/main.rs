//! Draw a square, circle and triangle on the screen using the embedded_graphics library over a 4
//! wire SPI interface.
//!

#![no_std]
#![no_main]

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyleBuilder,
};

use ssd1306::{prelude::*, Builder};

use arduboy::prelude::*;
use arduboy::spi;
//use arduboy::spi::prelude::*;
//use nb::block;

//use crate::spi;

use panic_halt as _;

#[arduboy::entry]
fn main() -> ! {
    let dp = arduboy::Peripherals::take().unwrap();

    let mut pins = arduboy::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut led0 = pins.led0.into_output(&mut pins.ddr);
    let mut led1 = pins.led1.into_output(&mut pins.ddr);
    let mut led2 = pins.led2.into_output(&mut pins.ddr);
    let mut speaker = pins.speaker1.into_output(&mut pins.ddr);
    let mut speaker2 = pins.speaker2.into_tri_state(&mut pins.ddr);

    speaker.set_high().void_unwrap();

    speaker2.set_low().void_unwrap();

    led0.set_high().void_unwrap();
    led1.set_high().void_unwrap();
    led2.set_high().void_unwrap();

    let leds = [led0.downgrade(), led1.downgrade(), led2.downgrade()];

    /*for i in 0..3 {    
        speaker.toggle().void_unwrap(); 
            
        leds[i].toggle().void_unwrap();
        leds[(i + 2) % 3].toggle().void_unwrap();
        arduboy::delay_ms(200);
    }*/

    let mut cs = pins.cs.into_tri_state(&mut pins.ddr); //Only one slave chip
    cs.set_low().void_unwrap();

    /*let mut arduboy_settings = spi::Settings{ 
        {data_order : spi::DataOrder::MostSignificantFirst,
        clock : spi::SerialClockRate::OscfOver4,
        mode : spi::MODE_0, }, // { 
            //polarity : spi::Polarity::IdleLow,
            //phase : spi::Phase::CaptureOnFirstTransition, }, },
    };*/


    // Create SPI interface.
    let (spi, mut led_rx) = spi::Spi::new(
        dp.SPI,
        pins.sck.into_output(&mut pins.ddr),
        pins.mosi.into_output(&mut pins.ddr),
        pins.miso.into_pull_up_input(&mut pins.ddr),
        pins.led_rx.into_output(&mut pins.ddr),
        
        spi::Settings::default(), /* {data_order : spi::DataOrder::MostSignificantFirst,
        clock : spi::SerialClockRate::OscfOver4,
        mode : spi::Mode { 
            polarity : spi::Polarity::IdleLow,
            phase : spi::Phase::CaptureOnFirstTransition, }, }, */
    );

    led_rx.set_low().unwrap();

    let dc = pins.dc.into_output(&mut pins.ddr);
    let mut delay = arduboy::Delay::new();
    let mut oled_rst = pins.oled_rst.into_output(&mut pins.ddr);


    let interface = ssd1306::prelude::SPIInterfaceNoCS::new(spi, dc);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    disp.reset(&mut oled_rst, &mut delay).unwrap();
    disp.init().unwrap();

    disp.clear();

    disp.display_on(true).unwrap(); 
   
    disp.flush().unwrap();

    //if result != DisplayError {
    //    led1.set_low().unwrap();
    //    arduboy::delay_ms(200);
    //}

    loop {

        disp.set_brightness(Brightness::BRIGHTEST).unwrap(); 
        arduboy::delay_ms(200);
        disp.set_brightness(Brightness::DIMMEST).unwrap();
        arduboy::delay_ms(200);
    
    }

    //if (result ==

    //match result {
    //    DisplayError => led0.toggle().void_unwrap(), 
    //    _ => led1.toggle().void_unwrap(),
    //};
     
    //printf(result);

    /*
    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // square
    Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(96, yoffset + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();
    */
    
    //loop {}
    
}

//#[exception]
//fn HardFault(ef: &ExceptionFrame) -> ! {
//    panic!("{:#?}", ef);
//}
