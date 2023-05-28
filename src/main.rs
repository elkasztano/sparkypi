use rppal::gpio::Gpio;
use clap::{Parser, Subcommand, ValueEnum};
use std::process::ExitCode;
use sparkypi::constants::*;
use sparkypi::*;


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
    /// control mains socket 'gmornxen'
    Xen {
        /// device
        #[arg(short, long = "device", value_enum)]
        device: XenDevice,

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
        /// protocol
        #[arg(long = "protocol", value_name = "PROTOCOL")]
        protocol: ProtocolChoice,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ProtocolChoice {
    Protocol1,
    Protocol2,
    Xen,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Device {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum XenDevice {
    A,
    B,
    C,
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

    let mut tm = Transmission::new();
    
    match &args.command {
        
        Commands::Socket { device, state } => {
            
            tm.sequence.push_str(DIP_SWITCH);
            tm.pulse_length = 310;
            tm.repeats = 10;
            tm.protocol = P1; 

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

        Commands::Xen { device, state } => {

            tm.sequence.push_str(XEN_PRE);
            tm.pulse_length = 580;
            tm.repeats = 10;
            tm.protocol = XEN;

            match device {
                XenDevice::A => match state {
                    State::On => tm.sequence.push_str(XEN_AON),
                    State::Off => tm.sequence.push_str(XEN_AOFF),
                },
                XenDevice::B => match state {
                    State::On => tm.sequence.push_str(XEN_BON),
                    State::Off => tm.sequence.push_str(XEN_BOFF),
                },
                XenDevice::C => match state {
                    State::On => tm.sequence.push_str(XEN_CON),
                    State::Off => tm.sequence.push_str(XEN_COFF),
                },
            }

            tm.sequence.push_str(XEN_POST);

        },

        Commands::Bell { device } => {

            tm.pulse_length = 175;
            tm.repeats = 5;
            tm.protocol = P1;
            
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
        
        Commands::Cus {s, pl, rp, protocol} => {
            tm.sequence.push_str(s);
            tm.pulse_length = *pl;
            tm.repeats = *rp;

            match protocol {
                ProtocolChoice::Protocol1 => tm.protocol = P1,
                ProtocolChoice::Protocol2 => tm.protocol = P2,
                ProtocolChoice::Xen => tm.protocol = XEN,
            }
        }
    
    }

    tm.send_to(&mut rc);

    ExitCode::SUCCESS
}
