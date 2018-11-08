extern crate serial;
extern crate spidev;
extern crate sysfs_gpio;
extern crate chrono;
extern crate ini;

// own uses
mod gps;
use gps::*;

mod image;
use image::*;

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
        Err(e) => { println!("Error: {}", e); std::process::exit(1); },
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
            GpsErrorType::GGA => println!("GPS: Error en la sentencia GGA"),
            GpsErrorType::RMC => println!("GPS: Error en la sentencia RMC"),
            GpsErrorType::Fix => println!("GPS: Error con el Fix"),
            GpsErrorType::Parse => println!("GPS: Error parseando los datos"),
            _ => {},
        }

    }
//
//    // test Image
//    let mut img: Image = Image::new(0, "ssdv", "/home/pi");
//    match img.capture() {
//        Ok(()) => println!("Capturada imagen {}", img.filename),
//        Err(e) => println!("Error tomando foto {}", e),
//    };
//
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
    lora.send("$$TELEMETRY TEST".as_bytes());
    lora.wait_packet_sent();

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
    telem.update(4807.038, 'N', 1131.000, 'E', 123.0, 80.2, 1.5, 
		4, 6.98, 1004.6, 25.3, 12.6, 0.9);     			

    println!("Telemetry: ");
    println!("{}", telem.aprs_string());


    std::process::exit(0);
}
