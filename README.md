# sparkypi

control 433 Mhz devices via the command line

## hardware requirements

You'll need

* a Raspberry Pi 4B or 5 (other models haven't been tested yet)
* a 433 Mhz transmitter module, like the FS1000A

Tested with the following target devices:

* Brennenstuhl RCS 1000SN remote controlled mains socket
* Gmornxen remote controlled mains socket
* PHYSEN doorbell

There are many different protocols to control 433 Mhz devices. The implemented presets 'P1' (tested) and 'P2' (untested) correspond to 'protocol 1' and 'protocol 2' in sui77's excellent rc-switch library for the Arduino ecosystem.
However, the 'Gmornxen' remote controlled socket switches seem to use a slightly different protocol, which is referred to as 'XEN' in my project.

I tried to make it as easy as possible to implement further custom protocols. If you have difficulties finding the right protocol for your target device, you might consider using an RTL SDR dongle to decode the signal of the corresponding counterpart of your device (e.g. the remote).

## hardware setup

Connect the pins of the transmitter module to the corresponding pins on the Raspberry Pi.

* VCC ⟶ 5V pin
* GND ⟶ GND pin
* data ⟶ GPIO17 on the Raspberry Pi by default, may be changed in 'constants.rs'

### target devices

Be sure to check the dip switches on the 'Brennenstuhl' remote controlled mains socket. (see 'constants.rs')

Doorbells usually must be programmed first in order to react to a certain binary sequence.

The 'Gmornxen' remote controlled socket switches need to be programmed as well.

## required operating system

sparkypi has currently been tested on

* Raspberry Pi OS 64 bit

* * Debian GNU/Linux 11 (bullseye)
* * Kernel: Linux 6.1.21-v8+

* Ubuntu 23.04
* * Kernel: Linux 6.2.0-1004-raspi

## compiling

`cargo build --release`

Compiling is preferably done on the Raspberry Pi itself. It should not take too long. Cross compiling is theoretically possible, although you might run into glibc compatibility issues.

## usage

### examples:

turn on socket A

`sparkypi socket -d a -s on`

turn off socket B

`sparkypi socket -d b -s off`

trigger doorbell 1

`sparkypi bell 1`

turn on 'Gmornxen' socket A

`sparkypi xen -d a -s on`

send custom binary sequence '000000010101000101011001' with a pulse length of 170 microseconds 5 times using protocol 1  
the 's' at the beginning of the string stands for the required sync bit at the beginning of each transmission  
finding the right pulse length usually requires some trial and error  

`sparkypi cus -s s000000010101000101011001 -p 170 -r 5 --protocol protocol1`
