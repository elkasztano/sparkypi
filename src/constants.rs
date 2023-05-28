pub const RC_PIN: u8 = 17;

// binary sequence for 'Brennenstuhl RCS 1000SN' 433 Mhz remote switch
// it is actually 'TriState', thats why only the following bit pairs will occur:
// '00', '01' and '10'
// other models may work similarly

// pairing: the remote and the sockets have dip switches that need to match
// first 10 bits: '00' if dip switch is up, '01' if dip switch is down, 5 dip switches alltogether
// change this according to your requirements
pub const DIP_SWITCH: &str = "s0000000000"; // example: first and last dip switch down, all others up
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
pub const DOORBELL_RING: [ &str; 10 ] = [
"s001100100101100101010110",
"s001010100011110010101101",
"s100011010110111011001111",
"s010101010001001101011001",
"s110110110110110100011010",
"s010100000001010001111010",
"s010111010111000110110000",
"s111000111000101010001101",
"s001100100101100101010011",
"s100101001001001110110011",
];

// 'Gmornxen' socket switch presets
pub const XEN_PRE: &str = "s10";

pub const XEN_POST: &str = "001100100001001100000000";

pub const XEN_AON: &str = "001110";
pub const XEN_AOFF: &str = "000001";

pub const XEN_BON: &str = "100110";
pub const XEN_BOFF: &str = "101110";

pub const XEN_CON: &str = "010110";
pub const XEN_COFF: &str = "011110";

// defining pulse length ('PL', microseconds) and repeats ('RP')

// finding the right pulse length usually requires a lot of fine tuning and may be a bit tricky
// the right pulse length for the Brennenstuhl RCS 1000SN seems to be 320 microseconds, and 190
// microseconds for the Physen doorbells respectively
pub const RC_PL: u16 = 320;
pub const DB_PL: u16 = 175;

// the number of repeats should be as low as reasonably achievable
// if it is too long other 433 Mhz devices like weather stations or even tire pressure control
// systems may get blocked
pub const RC_RP: u8 = 10;
pub const DB_RP: u8 = 5;
