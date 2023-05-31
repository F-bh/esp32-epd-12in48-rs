#![no_std]
#![no_main]

#[macro_use]
extern crate derive_new;

use crate::esp32_epd_12in48::{DisplayHalf, Epd12in48};
use esp_backtrace as _;
use hal::clock::ClockControl;
use hal::spi::SpiMode;
use hal::timer::TimerGroup;
use hal::{peripherals::Peripherals, prelude::*, Delay, Rtc, Spi, IO};

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

    /* let display = Epd12in48::new(
            spi,
            Delay::new(&clocks),
            DisplayHalf{
                reset_pin: (),
                command_pin: (),
                left_display: (),
                right_display: (),
            },
            /* bottom_half */);
    */
    loop {}
}
