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
    // test configuration file
    let mut config: Config = Config::new(CONFIG_FILE);
    match config.open() {
        Ok(()) => println!("{}", config.id),
        Err(e) => { println!("Error: {}", e.info); std::process::exit(1); },
    };


    // test gps
    let mut gps: GPS =  GPS::new(&config.gps_serial_port, config.gps_speed);
    gps.config().unwrap();
    match gps.update() {
        Ok(()) => {
            println!("{}", if gps.sats >=4 { "GPS FIX OK" } else { "NO GPS FIX" });
            println!("{}N, {}W", gps.decimal_latitude(), gps.decimal_longitude());
        },
        Err(e) => match e.error_type {
            GpsErrorType::Sats => println!("GPS: No hay suficientes sats"),
            GpsErrorType::GGA => println!("GPS: Error en la sentencia GGA: {}", gps.line_gga),
            GpsErrorType::RMC => println!("GPS: Error en la sentencia RMC"),
            GpsErrorType::Fix => println!("GPS: Error con el Fix"),
            GpsErrorType::Parse => println!("GPS: Error parseando los datos"),
            _ => {},
        }

    }

    // test Picture 
    let mut pic: Picture = Picture::new(
    	0, 
    	"ssdv", 
    	&(config.path_main_dir.clone() + &config.path_images_dir.clone())
	);

    // capture
    match pic.capture() {
        Ok(()) => println!("Capturada imagen {}", pic.filename),
        Err(e) => println!("Error tomando foto {:?}", e),
    };

    // capture small picture for ssdv
    match pic.capture_small(config.ssdv_name.clone(), config.ssdv_size.clone()) {
        Ok(()) => println!("Capturada imagen: {}", config.ssdv_name.clone()),
        Err(e) => println!("Error redimensionando foto {:?}", e),
    };

    // add info
    match pic.add_info(config.path_main_dir.clone()
                        + &config.path_images_dir.clone()
                        + &config.ssdv_name.clone(),
                    config.id.clone(),
                    config.subid.clone(),
                    config.msg.clone(),
                    format!("{}{}, {}{}, {}m",
                        gps.decimal_latitude(),
                        gps.ns,
                        gps.decimal_longitude(),
                        gps.ew,
                        gps.altitude,
                        )
                    ) {
        Ok(()) => println!("Modificada imagen."),
        Err(e) => println!("Error modificando foto {:?}", e),
    }

    // test ssdv
    let mut ssdv: SSDV = SSDV::new(
    		config.path_main_dir.clone()
                + &config.path_images_dir.clone()
                + &config.ssdv_name.clone(),
            config.path_main_dir.clone()
                + &config.path_images_dir.clone(),
			config.ssdv_name,
			config.id.clone(),
			0
			);
    match ssdv.encode() {
        Ok(()) => println!("Encodeado SSDV {}, paquetes: {}", 
                                ssdv.binaryname, ssdv.packets),
	    Err(e) => println!("Error encodeando SSDV: {:?}", e),
    };

    // get last ssdv packet
    let i = ssdv.packets - 1;
    match ssdv.get_packet(i) {
        Ok(p) => { 
            print!("Packet first 8 bytes: ");
            println!("{:x} {:x} {:x} {:x} {:x} {} {} {}", 
                                        p[0], p[1], p[2], p[3],
                                        p[4], p[5], p[6], p[7]);
        },
        Err(e) => println!("Error: {:?}", e),
    }

    // test temperature sensor
    let mut temp_sensor: DS18B20 = DS18B20::new(&config.temp_external_addr);
    println!("Temperature: {}", temp_sensor.read().unwrap());

    // test log
    let mut log: Log = Log::new("/home/pi/prueba.txt");

    log.init();
    log.log(LogType::Info, "Probando Log");
    log.log(LogType::Warn, "Advertencia!!!");
    log.log(LogType::Data, 
            &format!("External temperature: {}", temp_sensor.read().unwrap()));
    log.log(LogType::Error, "Vamos a morir!");

    // test led
    let mut led: LED = LED::new(config.led_pin);
    match led.init() {
        Ok(()) => {},
        Err(e) => println!("{}", e),
    }
    led.blink();
    led.err();

    // test mcp3002
    let mut mcp3002: Mcp3002 = Mcp3002::new(config.adc_cs, 0);
    mcp3002.init();
    match mcp3002.read(1) {
        Ok(n) => println!("ADC channel 1: {}", n),
        Err(e) => println!("Error reading ADC: {}", e),
    }

    // test baro
    let mut baro : Ms5607 =  Ms5607::new(config.baro_i2c_bus, config.baro_addr);
    baro.read_prom().unwrap();
    baro.update().unwrap();
    println!("Baro : {} mBar", baro.get_pres().unwrap());

    // test telemetry
    let mut telem: Telemetry = Telemetry::new(config.id, 
                            config.msg, 
                        config.separator);
    thread::sleep(Duration::from_millis(1500));
    telem.update(4807.038, 'N', 1131.000, 'E', 124.0, 80.2, 1.5, 
        4, 6.98, baro.get_pres().unwrap(), temp_sensor.read().unwrap(), 12.6);

    println!("Telemetry: ");
    println!("{}", telem.aprs_string());

    // test LoRa
    let mut lora: RF95 = RF95::new(config.lora_cs, config.lora_int_pin, false);
    match lora.init() {
        Ok(()) => println!("LoRa init ok"),
        Err(e) => { 
            println!("ERROR: {}", e);
            std::process::exit(1);
        },
    }

    lora.set_frequency(868.5);
    lora.set_tx_power(5);

    println!("Sending...");
    lora.send(telem.aprs_string().as_bytes());
    lora.wait_packet_sent();

    thread::sleep(Duration::from_millis(1000));

    println!("Sending image...");
    print!("Packet: ");
    // test sending a SSDV image
    for i in 0..ssdv.packets {
	    lora.send(&ssdv.get_packet(i).unwrap());
	    lora.wait_packet_sent();
        thread::sleep(Duration::from_millis(10));
	    print!("{}, ", &i);
	    io::stdout().flush().ok().expect("Could not flush stdout");
    }
    println!();

    std::process::exit(0);
}
