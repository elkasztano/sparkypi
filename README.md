# sparkypi
control 433 Mhz devices via the command line
## hardware requirements
You'll need
* a Raspberry Pi 4B (other models haven't been tested yet)
* a 433 Mhz transmitter module, like the FS1000A

Tested with the following target devices:
  * Brennenstuhl RCS 1000SN - remote controlled mains socket
  * PHYSEN Doorbell

It should be fairly easy to add functionality for other target devices as well.
There are many different protocols to control 433 Mhz devices. However, the devices I have been working with all seem to use the same protocol. What I have implemented here pretty much corresponds to 'protocol1' in the excellent 'rc-switch' library.
Other protocols may be implemented in the future.
## hardware setup
Connect the pins of the transmitter module to the corresponding pins on the Raspberry Pi.
* VCC ⟶ 5V pin
* GND ⟶ GND pin
* data ⟶ GPIO12 on the Raspberry Pi by default, may be changed in 'constants.rs'

### target devices
Be sure to check the dip switches on the remote controlled mains socket. (see 'constants.rs')

Doorbells usually must be programmed first in order to react to a certain binary sequence.
## required operating system
sparkypi has currently been tested on
* Raspberry Pi OS 64 bit
  * Debian GNU/Linux 11 (bullseye)
  * Kernel: Linux 6.1.21-v8+

* Ubuntu 23.04
  * Kernel: Linux 6.2.0-1004-raspi
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

send custom binary sequence '000000010101000101011001' with a pulse length of 170 microseconds 5 times  
the 's' at the beginning of the string stands for the required sync bit at the beginning of each transmission  
finding the right pulse length usually requires some trial and error  

`sparkypi cus -s s000000010101000101011001 -p 170 -r 5`
