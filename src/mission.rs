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


    ///////// MAIN LOOP /////////
    loop {

        // Telemetry
        for i in 0..config.packet_repeat {
            // Check for commands


            // Update sensor data
            // GPS
            match gps.update() {
                Ok(()) => {
                    log.log(
                        LogType::Data,
                        &format!("{}N, {}W, Sats: {}",
                            gps.decimal_latitude(), gps.decimal_longitude(), gps.sats
                        )
                    );
                },
                Err(e) => match e.error_type {
                    GpsErrorType::Sats => log.log(LogType::Warn, "GPS: No hay suficientes sats"),
                    GpsErrorType::GGA => log.log(
                        LogType::Warn,
                        &format!("GPS: Error en la sentencia GGA: {}", gps.line_gga)
                    ),
                    GpsErrorType::RMC => log.log(LogType::Warn, "GPS: Error en la sentencia RMC"),
                    GpsErrorType::Fix => log.log(LogType::Warn, "GPS: Error con el Fix"),
                    GpsErrorType::Parse => log.log(LogType::Warn, "GPS: Error parseando los datos"),
                    _ => {},
                }
            }

            // Baro
            baro.update().unwrap();
            log.log(LogType::Data, &format!("BARO: {}", baro.get_pres().unwrap()));

            // Temperatures
            let t_in = match temp_internal.read() {
                Ok(t) => { log.log(LogType::Data, &format!("TIN: {}", t)); t },
                Err(e) => { log.log(LogType::Warn, &format!("Error reading TIN: {}", e)); 9999.0 },
            };

            let t_out = match temp_external.read() {
                Ok(t) => { log.log(LogType::Data, &format!("TOUT: {}", t)); t },
                Err(e) => { log.log(LogType::Warn, &format!("Error reading TOUT: {}", e)); 9999.0 },
            };

            // Battery, read ADC channel and make conversion
            let adc_batt = match mcp3002.read(config.adc_vbatt) {
                Ok(n) => { log.log(LogType::Data, &format!("ADC0: {}", n)); n },
                Err(e) => { log.log(LogType::Warn, &format!("Error reading ADC: {}", e)); 0},
            };

            let mut vbatt: f32 = config.adc_v_mult * config.adc_v_divider
                * (adc_batt as f32 * 3.3/1023.0);
            log.log(LogType::Data, &format!("VBATT: {}", vbatt));

            // Create telemetry packet
            telem.update(
                gps.latitude,
                gps.ns,
                gps.longitude,
                gps.ew,
                gps.altitude,
                gps.heading,
                gps.speed,
                gps.sats,
                vbatt,
                baro.get_pres().unwrap(),
                t_in,
                t_out,
            );

            // Send telemetry
            log.log(LogType::Info, "Sending telemetry packet...");
            lora.send(telem.aprs_string().as_bytes());
            lora.wait_packet_sent();
            log.log(LogType::Info, "Telemetry packet sent.");
            led.blink();

            // Wait
            thread::sleep(Duration::from_millis(config.packet_delay as u64 *1000));

        }

        // Take picture

        // Take SSDV picture

        // Encode SSDV picture

        // Send SSDV

        // Wait
        thread::sleep(Duration::from_millis(config.packet_delay as u64 *1000));

    }
}

