/*!

Demo of AITEXM ROBOT TFT 2.4" touchscreen display with SD
card for MicroBit v2, from Alibaba Express.

Mostly derived from ili9341 crate example. Thanks for that.

Wiring:

    MB2   TFT

    P15   22 MOSI
    P13   23 SCK
    P08   21 DC
    P12   19 CS
    P09   20 RST
    +3.3  24 LED
    +3.3  17 VCC
    GND   18 GND

*/


#![no_main]
#![no_std]

use cortex_m_rt::entry;
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
//use embedded_hal::digital::{blocking::OutputPin, ErrorType, PinState};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use microbit::{
    board::Board,
    hal::{delay::Delay, gpio, spim},
    pac::spim0::frequency,
};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("starting");

    let board = Board::take().unwrap();

    let mut delay = Delay::new(board.SYST);

    let spi_mode = spim::Mode {
        polarity: spim::Polarity::IdleLow,
        phase: spim::Phase::CaptureOnFirstTransition,
    };
    // XXX Can probably be much faster.
    let spi_clock_rate = frequency::FREQUENCY_A::M8;
    let sck = board.pins.p0_17
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let mosi = board.pins.p0_13
        .into_push_pull_output(gpio::Level::Low)
        .degrade();
    let spi_pins = spim::Pins {
        sck: Some(sck),
        mosi: Some(mosi),
        miso: None,
    };
    let spi = spim::Spim::new(board.SPIM0, spi_pins, spi_clock_rate, spi_mode, 0xff);
    let dc = board.edge.e08
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let cs = board.edge.e12
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let spi_iface = SPIInterface::new(spi, dc, cs);

    let rst = board.edge.e09
        .into_push_pull_output(gpio::Level::High)
        .degrade();
    let mut lcd = Ili9341::new(
        spi_iface,
        rst,
        &mut delay,
        Orientation::Portrait,
        DisplaySize240x320,
    )
    .unwrap();

    rprintln!("set up");

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::with_alignment(
        "First line\nSecond line",
        Point::new(20, 30),
        style,
        Alignment::Center,
    )
    .draw(&mut lcd)
    .unwrap();

    rprintln!("drawn");

    loop {
        cortex_m::asm::wfe();
    }
}
