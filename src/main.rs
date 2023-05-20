use rppal::gpio::Gpio;
use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;
use sparkypi::constants::*;
use sparkypi::Transmission;


#[derive(Parser, Debug)]
#[command(author, version, about = "sparky pi - control 433 Mhz devices via command line", long_about = None)]

struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand,Debug)]
enum Commands {
    /// control mains socket
    Socket {
        /// device
        #[arg(short, long = "device", value_enum)]
        device: Device,

        /// state
        #[arg(short, long = "state", value_enum)]
        state: State,
    },
    
    /// trigger doorbell
    Bell {
        /// device id
        #[arg(value_name = "0..=255")]
        device: u8,
    },

    /// custom bit sequence
    Cus {
        /// sequence
        #[arg(short, long = "sequence", value_name = "STRING")]
        s: String,
        /// pulse length of the signal (µsecs)
        #[arg(short, long = "pulse-length", value_name = "U16")]
        pl: u16,
        /// number of repeats
        #[arg(short, long = "repeats", value_name = "U8")]
        rp: u8,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Device {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum State {
    On,
    Off,
}


fn main() -> ExitCode {

    let gpio = Gpio::new().unwrap();

    let mut rc = gpio.get(RC_PIN).unwrap().into_output();
    
    let args = Args::parse();

    let mut tm = Transmission {
        sequence: String::new(),
        pulse_length: 0,
        repeats: 0,
    };
    
    match &args.command {
        
        Commands::Socket { device, state } => {
            
            tm.sequence.push_str(DIP_SWITCH);
            tm.pulse_length = RC_PL;
            tm.repeats = RC_RP;

            match device {
                Device::A => tm.sequence.push_str(RC_SWITCH[0]),
                Device::B => tm.sequence.push_str(RC_SWITCH[1]),
                Device::C => tm.sequence.push_str(RC_SWITCH[2]),
                Device::D => tm.sequence.push_str(RC_SWITCH[3]),
            }

            match state {
                State::On => tm.sequence.push_str(RC_ON),
                State::Off => tm.sequence.push_str(RC_OFF),
            }
        
        },

        Commands::Bell { device } => {

            tm.pulse_length = DB_PL;
            tm.repeats = DB_RP;
            
            let seq = match DOORBELL_RING.get(*device as usize) {
                Some(sequence) => sequence,
                None => {
                    eprintln!("\x1b[0;7mrequested device undefined - exiting\x1b[0m");

                    eprint!("possible values:");

                    let mut i = 0;

                    for _ in DOORBELL_RING {
                        eprint!(" {}", i);
                        i += 1;
                    }

                    eprint!("\n");

                    return ExitCode::FAILURE;
                },
            };

            tm.sequence.push_str(seq);
        },
        
        Commands::Cus {s, pl, rp} => {
            tm.sequence.push_str(s);
            tm.pulse_length = *pl;
            tm.repeats = *rp;
        },
    
    }

    tm.send_to(&mut rc);

    ExitCode::SUCCESS
}
