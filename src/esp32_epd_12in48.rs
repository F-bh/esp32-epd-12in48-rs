use hal::ehal::spi::FullDuplex;
use hal::gpio::{InputPin, OutputPin};
use hal::prelude::_embedded_hal_blocking_spi_Transfer;
use hal::prelude::nb::block;
use hal::spi::{FullDuplexMode, Instance};
use hal::{spi, Delay, Error, Spi};
use readonly;

#[readonly::make]
#[derive(Copy, Clone)]
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
        op_code: 0x1,
        vals: [0; 5],
    };

    const BTST: Command<4> = Command {
        op_code: 0x6,
        vals: [0; 4],
    };

    const TRES: Command<4> = Command {
        op_code: 0x61,
        vals: [0; 4],
    };

    const DUSPI: Command<1> = Command {
        op_code: 0x15,
        vals: [0; 1],
    };

    const CDI: Command<2> = Command {
        op_code: 0x50,
        vals: [0; 2],
    };

    const TCON: Command<1> = Command {
        op_code: 0x60,
        vals: [0; 1],
    };

    const PWS: Command<1> = Command {
        op_code: 0xE3,
        vals: [0; 1],
    };

    const CCSET: Command<1> = Command {
        op_code: 0xE0,
        vals: [0; 1],
    };

    const TSSET: Command<1> = Command {
        op_code: 0xE5,
        vals: [0; 1],
    };
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
    // commands sent in the order of top -> bot; left -> right; vals should be in that order
    fn send_command_to_all<const LEN: usize>(
        &mut self,
        cmd: Command<LEN>,
        vals: [[u8; LEN]; 4],
    ) -> Option<Error> {
        self.top_half
            .left_display
            .send_command(&mut self.spi, cmd.with_vals(vals[0]))?;

        self.bottom_half
            .left_display
            .send_command(&mut self.spi, cmd.with_vals(vals[1]))?;

        self.top_half
            .right_display
            .send_command(&mut self.spi, cmd.with_vals(vals[2]))?;

        self.bottom_half
            .right_display
            .send_command(&mut self.spi, cmd.with_vals(vals[3]))?;
        return None;
    }

    // modeled after
    // https://github.com/waveshare/12.48inch-e-paper/blob/master/esp32/esp32-epd-12in48/src/utility/EPD_12in48.cpp#L68
    pub fn init(&mut self) -> Option<Error> {
        // send PSR commands
        self.send_command_to_all(Commands::PSR, [[0x1f]; 4])?;

        // set resolution
        self.send_command_to_all(
            Commands::TRES,
            [
                [0x02, 0x88, 0x01, 0xEC],
                [0x02, 0x88, 0x01, 0xEC],
                [0x02, 0x90, 0x01, 0xEC],
                [0x02, 0x90, 0x01, 0xEC],
            ],
        )?;

        // SPI settings
        self.send_command_to_all(Commands::DUSPI, [[0x20]; 4])?;

        // VCOM and data interval setting
        self.send_command_to_all(Commands::CDI, [[0x21, 0x07]; 4])?;

        // TCON settings
        self.send_command_to_all(Commands::TCON, [[0x22]; 4])?;

        // PWS settings
        self.send_command_to_all(Commands::PWS, [[0]; 4])?;

        // Cascade settings
        self.send_command_to_all(Commands::CCSET, [[0x03]; 4])?;

        // force temperature
        self.send_command_to_all(Commands::TSSET, [[0xE5]; 4])?;

        return None;
    }
}
