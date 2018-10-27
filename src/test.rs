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

mod ms5607;
use ms5607::*;

// CONFIGURATION
/////////////////

const CONFIG_FILE: &'static str = "/home/pi/nsx.cfg";

// MAIN
//////////////////

fn main() {
    // test configuration file
    let mut config: Config = Config::new(CONFIG_FILE);
    match config.open() {
        Ok(()) => println!("{:?}", config),
        Err(e) => { println!("Error: {}", e); std::process::exit(1); },
    };

    
    // test gps
    let mut gps: GPS =  GPS::new(&config.gps_serial_port, config.gps_speed);
    //gps.config(GPS_PORT_SETTINGS);
    gps.config();
   
    gps.update();
    println!("{}", if gps.sats >=4 { "GPS FIX OK" } else { "NO GPS FIX" });
    println!("{}N, {}W", gps.decimal_latitude(), gps.decimal_longitude());

    // test Image
    let mut img: Image = Image::new(0, "ssdv", "/home/pi");
    match img.capture() {
        Ok(()) => println!("Capturada imagen {}", img.filename),
        Err(e) => println!("Error tomando foto {}", e),
    };

    // test LoRa
    let mut lora: RF95 = RF95::new(config.lora_cs, config.lora_int_pin);
    if lora.init() {
        println!("LoRa init ok");
    }
    else {
        println!("ERROR: LoRa not found");
        std::process::exit(1);
    }

    //lora.set_frequency(868.5);
    //lora.set_tx_power(5);

    //println!("Sending...");
    //lora.send("$$TELEMETRY TEST".as_bytes());
    //lora.wait_packet_sent();

    // test temperature sensor
    let mut temp_sensor: DS18B20 = DS18B20::new(&config.temp_external_addr);
    println!("Temperature: {}", temp_sensor.read().unwrap());

    // test log
    let mut log: Log = Log::new("/home/pi/prueba.txt");

    log.init();
    log.log(LogType::Info, "Probando Log");
    log.log(LogType::Warn, "Advertencia!!!");
    log.log(LogType::Data, &format!("External temperature: {}", temp_sensor.read().unwrap()));
    log.log(LogType::Error, "Vamos a morir!");

    // test led
    let mut led: LED = LED::new(config.led_pin);
    match led.init() {
        Ok(()) => {},
        Err(e) => println!("{}", e),
    }
    led.blink();
    led.err();

    std::process::exit(0);
}
