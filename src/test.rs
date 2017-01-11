extern crate serial;
extern crate spidev;
extern crate sysfs_gpio;
extern crate chrono;

mod gps;
use gps::*;

mod image;
use image::*;

mod rf95;
use rf95::*;

mod ds18b20;
use ds18b20::*;

mod log;
use log::*;

// CONFIGURATION
/////////////////

static GPS_PORT_NAME: &'static str = "/dev/ttyAMA0";

const GPS_PORT_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone
};


// MAIN
//////////////////

fn main() {
    
    // test gps
    let mut gps: GPS =  GPS::new(GPS_PORT_NAME);
    gps.config(GPS_PORT_SETTINGS);
   
    //gps.update();
    //println!("{}", if gps.sats >=4 { "GPS FIX OK" } else { "NO GPS FIX" });
    //println!("{}N, {}W", gps.decimal_latitude(), gps.decimal_longitude());

    // test Image
    let mut img: Image = Image::new(0, "ssdv", "/home/pi");
    if img.capture() {
	println!("Capturada imagen {}", img.filename);
    }
    else {
	println!("Error");
    }

    // test LoRa
    let mut lora: RF95 = RF95::new(0, 25);
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
    let mut temp_sensor: DS18B20 = DS18B20::new("28-031682a91bff");
    println!("Temperature: {}", temp_sensor.read());

    // test log
    let mut log: Log = Log::new("prueba.txt");

    log.init();
    log.log(LogType::Info, "Probando Log");
    log.log(LogType::Warn, "Advertencia!!!");

    std::process::exit(0);
}
