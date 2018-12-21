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
