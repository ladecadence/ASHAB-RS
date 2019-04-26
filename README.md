# ASHAB-RS

Tracking and telemetry code for the ASHAB Payloads. Rust Code.

Transmits position/telemetry using digital packets and images using SSDV.
Runs on Raspberry Pi with raspbian, ssdv, and imagemagick.

Designed to run on a raspberry pi zero with the stratozero board:
http://wiki.ashab.space/doku.php?id=stratozero

See: http://wiki.ashab.space/doku.php?id=nearspacetwo (Spanish)

## Modules:

* config.rs: Main program and modules configuration
* gps.rs : GPS control and decoding
* rf95.rs : RF95 LoRa Radio module control
* ssdv.rs : ssdv program interface
* ds18b20.rs : DS18B20 temperature sensors
* picture.rs : Image capture and SSDV generation
* led.rs : Status LED methods
* log.rs : Logging system
* mcp3002.rs : SPI MCP3002 analog to digital converter
* ms5607.rs : i2c barometer
* telemetry.rs: Telemetry packets creation
* test.rs: simple test of all the submodules
* mission.rs : Main mission code

## Installation and running

You'll need a RaspberryPi Zero running Raspbian with the following software:

* Imagemagick: $ sudo apt install imagemagick
* Rust: (For development and compilation) $ curl https://sh.rustup.rs -sSf | sh
* ssdv: https://github.com/fsphil/ssdv -> make && sudo make install

The software can be built using cargo, the rust's package manager:

```
$ git clone https://github.com/ladecadence/ASHAB-RS.git
$ cd ASHAB-RS/
$ cargo build

```

Then put the configuration file (nsx.cfg) in your /home/pi folder 
(or change the path in mission.rs) and create a folder for the data (defined in nsx.cfg)
and inside it a pictures/ folder

* /home/pi
* nsx.cfg
* MISSION/
  * pictures/

Then you'll need to run the binary after the raspberry finishes booting.


## Config file

The configuration file contains all the parameters needed to configure the mission and the hardware used.
It's format is the typical INI file, divided in several sections for each mission parameter and hardware.

* [mission] : mission identifiers, telemetry format and intervals.
* [gpio]: general GPIO used by the software (check StratoZero pins).
* [gps]: gps port and baudrate.
* [lora]: lora radio SPI bus and configuration.
* [adc]: Analog to digital converter bus and battery calibration.
* [temp]: temperature sensors addresses.
* [baro]: barometer i2c configuration.
* [paths]: general paths for mission logs and images
* [ssdv]: SSDV image configuration.

An example of a config file:

```
[mission]
id = MISSION
subid = /ID
msg = High Altitude Balloon mission\ninfo@foo.bar
separator = /
packet_repeat = 20 
packet_delay = 5 

[gpio]
batt_enable_pin = 24
led_pin = 17
pwr_pin = 26

[gps]
serial_port = /dev/ttyAMA0
speed = 9600

[lora]
cs = 0
int_pin = 25
freq = 868.5
low_pwr = 5
high_pwr = 20

[adc]
cs = 1
vbatt = 0
v_divider = 3.2
v_mult = 1.07

[temp]
internal_addr = 28-041682c3dbff
external_addr = 28-0316b56c09ff 

[baro]
i2c_bus = 1
i2c_addr = 0x77

[paths]
main_dir = /home/pi/MISSION/
images_dir = images/
log = log.txt

[ssdv]
size = 320x240
name = ssdv.jpg

```

