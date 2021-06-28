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
* Rust: (For development and compilation on the raspberry) $ curl https://sh.rustup.rs -sSf | sh
* ssdv: https://github.com/fsphil/ssdv -> make && sudo make install

Also in raspi-config, you need to enable the i2c, spi, 1-wire and serial interfaces (serial interface WITHOUT console output).

The software can be built using cargo, the rust's package manager:

```
$ git clone https://github.com/ladecadence/ASHAB-RS.git
$ cd ASHAB-RS/
$ cargo build

```

You can also build the binary on your host computer, in linux install the cross-compiler for raspberry pi:

```
$ sudo apt install install gcc-arm-linux-gnueabihf 

```

And build using cargo cross compiling abilities

```
$ cargo build --target=arm-unknown-linux-gnueabihf

```

Then put the configuration file (nsx.cfg) in your /home/pi folder 
(or change the path in mission.rs) and create a folder for the data (defined in nsx.cfg)
and inside it a pictures/ folder

* /home/pi
* nsx.cfg
* MISSION/
  * pictures/

Then you'll need to run the binary after the raspberry finishes booting.
For this you can use the included ashabpi.service systemd file, modify it to your binary path, copy it to /lib/systemd/system/ chmod 644 it, and run

```
$ sudo systemctl daemon-reload
$ sudo systemctl enable ashabpi.service
```

## RTC

If using the RTC you need to configure the raspberry for it. First check the RTC is available using i2cdetect (from i2c-tools package):

```
$ sudo i2cdetect -y 1

     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f                                                                     
00:          -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
40: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --                                                                     
60: -- -- -- -- -- -- -- -- 68 -- -- -- -- -- -- --                                                                     
70: -- -- -- -- -- -- -- 77                           

```

You should see a 68 address present. Then install the rtc module and enable the rtc:
```
$ sudo modprobe rtc-ds1307
$ sudo su
# echo ds1307 0x68 > /sys/class/i2c-adapter/i2c-1/new_device
# exit
```

Then test that the RTC is alive:

```
$ sudo hwclock -r
2000-01-01 00:15:33.980780+00:00
```
Now set the system date using the date command or connecting the raspberry to the internet, and then set the RTC date:

```
$ sudo hwclock -w
$ sudo hwclock -r
2019-09-13 14:35:00.538018+02:00
```

Now to make this changes permanent, add the RTC module to the /etc/modules file

```
$ sudo su
# echo rtc-ds1307 >> /etc/modules
# exit
```

And edit /etc/rc.local to enable and update the clock on startup:

```
$ sudo nano /etc/rc.local

```

and add this lines just **before** exit 0:

```
echo ds1307 0x68 > /sys/class/i2c-adapter/i2c-1/new_device
sudo hwclock -s

date
```

Now each time the raspberry starts, you should have a correct date and time.

## Config file

The configuration file contains all the parameters needed to configure the mission and the hardware used. It will be stored in XDG_CONFIG_HOME/ashab-rs/ashab-rs.toml (usually $HOME/.config/..)
It's format is a TOML file, with entries for each mission parameter and hardware:

  * id: Mission main identifier (amateur radio callsign for example). Better if compatible with APRS.
  * subid: Mission sub identifuer (APRS callsign notation for example).
  * msg: Message added at the end of each telemetry packet and to the SSDV pictures.
  * separator: Separator character between fields in the telemetry packet (default "/" to make it compatible with APRS packets)
  * packet_repeat: number of telemetry packets to send between SSDV images
  * packet_delay: seconds between telemetry packets.

  * batt_enable_pin: GPIO (broadcom notation) used to enable and disable battery reading (consumes power). GPIO 24 on StratoZero board.
  * led_pin: GPIO used for status LED. GPIO 17 on StatoZero.
  * pwr_pin: GPIO used to configure RF power, high or low. GPIO 26 on StratoZero board.

  * gps_serial_port: serial port device (/dev/ttyAMA0, etc)
  * gps_speed: GPS baudrate (like 9600).

  * lora_cs: Chip Select channel for SPI bus. LoRa Radio on StatoZero board uses CS 0.
  * lora_int_pin: LoRa Radio interrupt pin. Used to check received packets or radio activity. StratoZero uses GPIO 25.
  * lora_freq: LoRa Radio output frequency (in MHz).
  * lora_low_pwr: Low RF power, useful when testing on ground. See high_pwr.
  * lora_high_pwr: High RF power, used when flying. RF95 LoRa radios used in the StatoZero boards minimun and maximum power leves are 5-20.

  * adc_cs: Chip Select channel for SPI bus. MCP3002 ADC on StratoZero board uses CS 1.
  * adc_vbatt: ADC channel used to read battery. StratoZero board uses ADC channel 0.
  * adc_v_divider: voltage divider ratio. StratoZero uses a 3.2 ratio for using 2 cell LiPo battery.
  * adc_v_mult: multiplier to calibrate battery readings.

  * temp_internal_addr &
  * temp_external_addr: 1-Wire bus addresses of the DS18B20 temperature sensors. You can find them in /sys/bus/w1/devices/

  * baro_i2c_bus: Raspberry Pi has 2 i2c buses. External i2c bus (the one present in the GPIO pins) is bus 1.
  * baro_i2c_addr: i2c address of the baraometer. StratoZero uses a MS5607 sensor with address 0x77

  * path_main_dir: main path where we store logs, images, etc.
  * path_images_dir: image storage relative path (to main_dir)
  * path_log_prefix: prefix of the log file name, will be completed with datetime and ".log" extension.

  * ssdv_size: SSDV image resolution. WIDTHxHEIGHT pixels, like 640x480.
  * ssdv_name: temporary filename for the SSDV image conversion.

An example of a config file:

```

id = 'MISSION'
subid = '/ID'
msg = 'High Altitude Balloon mission\ninfo@foo.bar'
separator = '/'
packet_repeat = 20 
packet_delay = 5 

batt_enable_pin = 24
led_pin = 17
pwr_pin = 26

gps_serial_port = '/dev/ttyAMA0'
gps_speed = 9600

lora_cs = 0
lora_int_pin = 25
lora_freq = 868.5
lora_low_pwr = 5
lora_high_pwr = 20

adc_cs = 1
adc_vbatt = 0
adc_v_divider = 3.2
adc_v_mult = 1.07

temp_internal_addr = '28-041682c3dbff'
temp_external_addr = '28-0316b56c09ff' 

baro_i2c_bus = 1
baro_i2c_addr = 0x77

path_main_dir = '/home/pi/MISSION/'
path_images_dir = 'images/'
path_log_prefix = 'missionlog'


ssdv_size = '320x240'
ssdv_name = 'ssdv.jpg'

```

