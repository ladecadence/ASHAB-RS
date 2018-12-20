// (C) 2018 David Pello Gonzalez for ASHAB
//
// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation, either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.
// If not, see <http://www.gnu.org/licenses/>.


extern crate serial;
extern crate spidev;
extern crate sysfs_gpio;
extern crate chrono;
extern crate ini;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::time::Duration;
use std::thread;
use std::io::prelude::*;
use std::io;

// own uses
mod gps;
use gps::*;

mod picture;
use picture::*;

mod rf95;
use rf95::*;

mod ds18b20;
use ds18b20::*;

mod config;
use config::*;

mod log;
use log::*;

mod led;
use led::*;

mod mcp3002;
use mcp3002::*;

mod ms5607;
use ms5607::*;

mod telemetry;
use telemetry::*;

mod ssdv;
use ssdv::*;

// CONFIGURATION
/////////////////

const CONFIG_FILE: &'static str = "/home/pi/nsx.cfg";

// MAIN
//////////////////

fn main() {
    // Test configuration file
    let mut config: Config = Config::new(CONFIG_FILE);
    match config.open() {
        Ok(()) => println!("{}", config.id),
        Err(e) => { println!("Error: {}", e.info); std::process::exit(1); },
    };

    // Start logging
    let mut log: Log = Log::new(&config.path_log);
    log.init();
    log.log(LogType::Info, "NSX starting.");

    // Ok, now start the peripherals using parameters from config file
    // GPS
    let mut gps: GPS =  GPS::new(&config.gps_serial_port, config.gps_speed);
    gps.config().unwrap();

    // Status LED
    let mut led: LED = LED::new(config.led_pin);
    match led.init() {
        Ok(()) => {},
        Err(e) => println!("{}", e),
    }
    led.blink();

    // ADC
    let mut mcp3002: Mcp3002 = Mcp3002::new(config.adc_cs, 0);
    mcp3002.init();

    // Barometer
    let mut baro : Ms5607 =  Ms5607::new(config.baro_i2c_bus, config.baro_addr);
    baro.read_prom().unwrap();

    // Temperature sensors
    let mut temp_internal: DS18B20 = DS18B20::new(&config.temp_internal_addr);
    let mut temp_external: DS18B20 = DS18B20::new(&config.temp_external_addr);

    // LoRa radio
    let mut lora: RF95 = RF95::new(config.lora_cs, config.lora_int_pin, false);
    match lora.init() {
        Ok(()) => println!("LoRa init ok"),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        },
    }

    lora.set_frequency(config.lora_freq);
    lora.set_tx_power(config.lora_low_pwr);

    // Telemetry object
    let mut telem: Telemetry = Telemetry::new(config.id,
        config.msg,
        config.separator
    );

    // Picture (camera control) object
    let mut pic: Picture = Picture::new(
    	0,
    	"ssdv",
    	&(config.path_main_dir.clone() + &config.path_images_dir.clone())
	);

    // Ok, now get time from GPS and update system time


    ///////// MAIN LOOP
    while 1 {
        // Check for commands

        // Telemetry
        for i in 0..config.packet_repeat {

            // Update sensor data

            // Create telemetry packet

            // Send telemetry

            // Wait
            thread::sleep(Duration::from_millis(config.packet_delay*1000));

        }

        // Take picture

        // Take SSDV picture

        // Encode SSDV picture

        // Send SSDV

        // Wait
        thread::sleep(Duration::from_millis(config.packet_delay*1000));

    }
}

 
