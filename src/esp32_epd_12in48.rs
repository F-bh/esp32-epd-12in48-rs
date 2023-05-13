use hal::gpio::{InputPin, OutputPin};
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

enum Display<BusyPin: InputPin, ChipSelectPin: OutputPin> {
    WIDE(DisplayInternals<BusyPin, ChipSelectPin>),
    SLIM(DisplayInternals<BusyPin, ChipSelectPin>),
}

struct DisplayInternals<BusyPin: InputPin, ChipSelectPin: OutputPin> {
    busy_pin: BusyPin,
    chip_select_pin: ChipSelectPin,
}

impl<BusyPin: InputPin, ChipSelectPin: OutputPin> Display<BusyPin, ChipSelectPin> {
    fn width(&self) -> u32 {
        match self {
            Display::WIDE(_) => 656,
            Display::SLIM(_) => 648,
        }
    }

    fn height() -> u32 {
        492
    }
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
    left_display: Display<BusyPinL, ChipSelectPinL>,
    right_display: Display<BusyPinR, ChipSelectPinR>,
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
    top_half:
        DisplayHalf<TopResetPin, TopDataCommandPin, TopLBusyPin, TopLCSPin, TopRBusyPin, TopRCSPin>,
    bottom_half:
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
