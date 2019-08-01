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

Also in raspi-config, you need to enable the i2c, spi, 1-wire and serial interfaces (serial interface WITHOUT console output).

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
For this you can use the included ashabpi.service systemd file, copy it to /lib/systemd/system/m chmod 644 it, and run

```
sudo systemctl daemon-reload
sudo systemctl enable ashabpi.service
```




## Config file

The configuration file contains all the parameters needed to configure the mission and the hardware used.
It's format is the typical INI file, divided in several sections for each mission parameter and hardware.

* [mission] : Mission identifiers, telemetry format and intervals.
  * id: Mission main identifier (amateur radio callsign for example). Better if compatible with APRS.
  * subid: Mission sub identifuer (APRS callsign notation for example).
  * msg: Message added at the end of each telemetry packet and to the SSDV pictures.
  * separator: Separator character between fields in the telemetry packet (default "/" to make it compatible with APRS packets)
  * packet_repeat: number of telemetry packets to send between SSDV images
  * packet_delay: seconds between telemetry packets.
* [gpio]: General GPIO used by the software (check StratoZero pins).
  * batt_enable_pin: GPIO (broadcom notation) used to enable and disable battery reading (consumes power). GPIO 24 on StratoZero board.
  * led_pin: GPIO used for status LED. GPIO 17 on StatoZero.
  * pwr_pin: GPIO used to configure RF power (high and low). GPIO 26 on StratoZero).
* [gps]: GPS port and baudrate.
  * serial_port: serial port device (/dev/ttyAMA0, etc)
  * speed: GPS baudrate (like 9600).
* [lora]: LoRa radio SPI bus and configuration.
  * cs: Chip Select channel for SPI bus. LoRa Radio on StatoZero board uses CS 0.
  * int_pin: LoRa Radio interrupt pin. Used to check received packets or radio activity. StartoZero uses GPIO 25.
  * freq: LoRa Radio output frequency.
  * low_pwr: Low RF power, useful when testing on ground.
  * high_pwr: High RF power, used when flying. RF95 LoRa radios used in the StatoZero boards minimun and maximum power leves are 5-20.
* [adc]: Analog to digital converter bus and battery calibration.
  * cs: Chip Select channel for SPI bus. MCP3002 ADC on StratoZero board uses CS 1.
  * vbatt: ADC channel used to read battery. StratoZero board uses ADC channel 0.
  * v_divider: voltage divider ratio. StratoZero uses a 3.2 ratio for using 2 cell LiPo battery.
  * v_mult: multiplier to calibrate battery readings.
* [temp]: Temperature sensors addresses.
  * internal_addr &
  * external_addr: 1-Wire bus addresses of the DS18B20 temperature sensors. You can find the in /sys/bus/w1/devices/
* [baro]: Barometer i2c configuration.
  * i2c_bus: Raspberry Pi has 2 i2c buses. External i2c bus (the one present in the GPIO pins) is bus 1.
  * i2c_addr: i2c address of the baraometer. StratoZero uses a MS5607 sensor with address 0x77
* [paths]: General paths for mission logs and images
  * main_dir: main path where we store logs, images, etc.
  * images_dir: image storage relative path (to main_dir)
  * log: log file name.
* [ssdv]: SSDV image configuration.
  * size: SSDV image resolution. WIDTHxHEIGH, like 640x480.
  * name: temporary filename for the SSDV image conversion.

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

