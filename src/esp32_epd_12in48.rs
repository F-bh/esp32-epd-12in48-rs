use hal::gpio::{InputPin, OutputPin};
use hal::prelude::_embedded_hal_spi_FullDuplex;
use hal::spi::{FullDuplexMode, Instance, SpiMode};
use hal::{Delay, Spi};

enum Command {
    //TODO Implement
}

/**
      WIDE           SLIM
|--------------|--------------|
|      S2      |     M2       |
|    (TopL)    |    (TopR)    |
|--------------|--------------|
|      M1      |     S1       |
|    (BotL)    |   (BotR)     |
|--------------|--------------|

 **/

enum Display<BusyPin: InputPin, ChipSelectPin: OutputPin, ResetPin: OutputPin> {
    WIDE(DisplayInternals<BusyPin, ChipSelectPin, ResetPin>),
    SLIM(DisplayInternals<BusyPin, ChipSelectPin, ResetPin>),
}

struct DisplayInternals<BusyPin: InputPin, ChipSelectPin: OutputPin, ResetPin: OutputPin> {
    busy_pin: BusyPin,
    chip_select_pin: ChipSelectPin,
    reset_pin: ResetPin,
}

impl<'spi, BusyPin: InputPin, ChipSelectPin: OutputPin, ResetPin: OutputPin>
    Display<BusyPin, ChipSelectPin, ResetPin>
{
    pub fn width(&self) -> u32 {
        match self {
            Display::WIDE(_) => 656,
            Display::SLIM(_) => 648,
        }
    }

    pub fn height() -> u32 {
        492
    }


    // psr: Panel Setting
    pub fn sendCommand<SPIPeripheral: Instance>(
        &mut self,
        spi: &mut Spi<'_, SPIPeripheral, FullDuplexMode>,
        operation: u8,
        values: [u8;9],
    ) {
        let internals = match self {
            Display::WIDE(internals) => internals,
            Display::SLIM(internals) => internals,
        };

        internals.chip_select_pin.set_output_high(false);
        spi.send(options).expect("failed to send psr command byte");
        spi.send(options).expect("failed to send psr command data");
    }

    pub fn psr<SPIPeripheral: Instance>(
        &mut self,
        spi: &mut Spi<'_, SPIPeripheral, FullDuplexMode>,
        options: u8,
    )
}

pub struct DisplayHalf<
    ResetPin: OutputPin,
    CommandPin: OutputPin,
    BusyPinL: InputPin,
    ChipSelectPinL: OutputPin,
    BusyPinR: InputPin,
    ChipSelectPinR: OutputPin,
> {
    reset_pin: ResetPin,
    command_pin: CommandPin,
    left_display: Display<BusyPinL, ChipSelectPinL, ResetPin>,
    right_display: Display<BusyPinR, ChipSelectPinR, ResetPin>,
}

#[derive(new)]
pub struct Epd12in48<
    'spi,
    SPIPeripheral,
    SPIMode,
    TopLCSPin: OutputPin,
    TopRCSPin: OutputPin,
    BotLCSPin: OutputPin,
    BotRCSPin: OutputPin,
    TopLBusyPin: InputPin,
    TopRBusyPin: InputPin,
    BotLBusyPin: InputPin,
    BotRBusyPin: InputPin,
    TopResetPin: OutputPin,
    BotResetPin: OutputPin,
    TopDataCommandPin: OutputPin,
    BotDataCommandPin: OutputPin,
> {
    spi: Spi<'spi, SPIPeripheral, SPIMode>,
    delay: Delay,
    pub top_half:
        DisplayHalf<TopResetPin, TopDataCommandPin, TopLBusyPin, TopLCSPin, TopRBusyPin, TopRCSPin>,
    pub bottom_half:
        DisplayHalf<BotResetPin, BotDataCommandPin, BotLBusyPin, BotLCSPin, BotRBusyPin, BotRCSPin>,
}

impl<
        'spi,
        SPIPeripheral,
        SPIMode,
        TopLCSPin: OutputPin,
        TopRCSPin: OutputPin,
        BotLCSPin: OutputPin,
        BotRCSPin: OutputPin,
        TopLBusyPin: InputPin,
        TopRBusyPin: InputPin,
        BotLBusyPin: InputPin,
        BotRBusyPin: InputPin,
        TopResetPin: OutputPin,
        BotResetPin: OutputPin,
        TopDataCommandPin: OutputPin,
        BotDataCommandPin: OutputPin,
    >
    Epd12in48<
        'spi,
        SPIPeripheral,
        SPIMode,
        TopLCSPin,
        TopRCSPin,
        BotLCSPin,
        BotRCSPin,
        TopLBusyPin,
        TopRBusyPin,
        BotLBusyPin,
        BotRBusyPin,
        TopResetPin,
        BotResetPin,
        TopDataCommandPin,
        BotDataCommandPin,
    >
{
}
