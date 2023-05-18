// GPIO-Pin the 433 Mhz TX module is connected to
pub const RC_PIN: u8 = 12;

// binary sequence for 'Brennenstuhl RCS 1000SN' 433 Mhz remote switch
// it is actually 'TriState', thats why only the following bit pairs will occur:
// '00', '01' and '10'
// other models may work similarly

// pairing: the remote and the sockets have dip switches that need to match
// first 10 bits: '00' if dip switch is up, '01' if dip switch is down, 5 dip switches alltogether
// change this according to your requirements
pub const DIP_SWITCH: &str = "s0100000001"; // example: first and last dip switch down, all others up
// the 's' stands for the sync bit at the beginning of each transmission

// next 10 bits: a sequence of '00' and '01' corresponding to socket switches A,B,C,D
pub const RC_SWITCH: [ &str; 4 ] = [ 
"0001010101", // switch A
"0100010101", // switch B
"0101000101", // switch C
"0101010001"  // switch D
];

// last 4 bits: '0001' ON, '0100' OFF
pub const RC_ON: &str = "0001";

pub const RC_OFF: &str = "0100";

// example codes for 'PHYSEN' doorbell
// the doorbell needs to be programmed first in order to react to the corresponding binary sequence
// note that one device can react to multiple different codes
pub const DOORBELL_RING: [ &str; 3 ] = [
"s000011000010111110011010",
"s000111010110111000001111",
"s010101011111001101011001"
];

// defining pulse length ('PL', microseconds) and repeats ('RP')

// finding the right pulse length usually requires a lot of fine tuning and may be a bit tricky
// the right pulse length for the Brennenstuhl RCS 1000SN seems to be 320 microseconds, and 190
// microseconds for the Physen doorbells respectively
pub const RC_PL: u64 = 320;
pub const DB_PL: u64 = 190;

// the number of repeats should be as low as reasonably achievable
// if it is too long other 433 Mhz devices like weather stations or even tire pressure control
// systems may get blocked
pub const RC_RP: usize = 10;
pub const DB_RP: usize = 5;
