#![no_std]
#![no_main]

use crate::esp32_epd_12in48::Epd12in48;
use esp_backtrace as _;
use hal::clock::ClockControl;
use hal::spi::SpiMode;
use hal::timer::TimerGroup;
use hal::{peripherals::Peripherals, prelude::*, Rtc, Spi, IO};

mod esp32_epd_12in48;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut timer_group = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );

    //disable watchdogs
    timer_group.wdt.disable();
    Rtc::new(peripherals.RTC_CNTL).rwdt.disable();

    let mut spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        io.pins.gpio5,
        io.pins.gpio1,
        8000u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let display = Epd12in48::new(
        spi,
        io.pins.gpio15,
        io.pins.gpio16,
        io.pins.gpio17,
        io.pins.gpio18,
        io.pins.gpio19,
        io.pins.gpio20,
        io.pins.gpio21,
        io.pins.gpio22,
        io.pins.gpio23,
        io.pins.gpio24,
        io.pins.gpio25,
        io.pins.gpio26,
        &clocks,
    );

    loop {}
}
