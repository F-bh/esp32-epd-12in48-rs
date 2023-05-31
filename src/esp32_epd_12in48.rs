use hal::ehal::spi::FullDuplex;
use hal::gpio::{InputPin, OutputPin};
use hal::prelude::_embedded_hal_blocking_spi_Transfer;
use hal::prelude::nb::block;
use hal::spi::{FullDuplexMode, Instance};
use hal::{spi, Delay, Spi};
use readonly;

#[readonly::make]
struct Command<const VALS: usize> {
    #[readonly]
    pub op_code: u8,
    #[readonly]
    pub vals: [u8; VALS],
}

impl<const VALS: usize> Command<VALS> {
    pub fn with_vals(mut self, vals: [u8; VALS]) -> Self {
        self.vals = vals;
        return self;
    }
}

struct Commands {}

impl Commands {
    const PSR: Command<1> = Command {
        op_code: 0,
        vals: [0],
    };

    const PWR: Command<5> = Command {
        op_code: 1,
        vals: [0; 5],
    };

    /*impl Commands {

    }*/
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

enum Display<
    BusyPin: InputPin,
    ChipSelectPin: OutputPin,
    CommandPin: OutputPin,
    ResetPin: OutputPin,
> {
    WIDE(DisplayInternals<BusyPin, ChipSelectPin, CommandPin, ResetPin>),
    SLIM(DisplayInternals<BusyPin, ChipSelectPin, CommandPin, ResetPin>),
}

struct DisplayInternals<
    BusyPin: InputPin,
    ChipSelectPin: OutputPin,
    CommandPin: OutputPin,
    ResetPin: OutputPin,
> {
    busy_pin: BusyPin,
    chip_select_pin: ChipSelectPin,
    command_pin: CommandPin,
    reset_pin: ResetPin,
}

impl<
        'spi,
        BusyPin: InputPin,
        ChipSelectPin: OutputPin,
        CommandPin: OutputPin,
        ResetPin: OutputPin,
    > Display<BusyPin, ChipSelectPin, CommandPin, ResetPin>
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

    fn send_command<SPIPeripheral: Instance, const CMD_LEN: usize>(
        &mut self,
        spi: &mut Spi<SPIPeripheral, FullDuplexMode>,
        mut command: Command<CMD_LEN>,
    ) -> Option<spi::Error> {
        let internals = match self {
            Display::WIDE(internals) => internals,
            Display::SLIM(internals) => internals,
        };

        internals.chip_select_pin.set_output_high(false);
        internals.command_pin.set_output_high(true);
        if let Err(err) = block!(spi.send(command.op_code)) {
            return Some(err);
        }
        internals.command_pin.set_output_high(false);

        if let Err(err) = spi.transfer(&mut command.vals) {
            return Some(err);
        }

        internals.chip_select_pin.set_output_high(true);
        return None;
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
    left_display: Display<BusyPinL, ChipSelectPinL, CommandPin, ResetPin>,
    right_display: Display<BusyPinR, ChipSelectPinR, CommandPin, ResetPin>,
}

#[derive(new)]
pub struct Epd12in48<
    'spi,
    SPIPeripheral,
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
    spi: Spi<'spi, SPIPeripheral, FullDuplexMode>,
    delay: Delay,
    pub top_half:
        DisplayHalf<TopResetPin, TopDataCommandPin, TopLBusyPin, TopLCSPin, TopRBusyPin, TopRCSPin>,
    pub bottom_half:
        DisplayHalf<BotResetPin, BotDataCommandPin, BotLBusyPin, BotLCSPin, BotRBusyPin, BotRCSPin>,
}

impl<
        'spi,
        SPIPeripheral: Instance,
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
    pub fn init(&mut self) {
        let mut x = Commands::PSR;
        x.vals = [2; 1];
        self.top_half
            .left_display
            .send_command(&mut self.spi, Commands::PSR.with_vals([0x1f]));
    }
}
