extern crate serial;
extern crate spidev;

use std::io::prelude::*;

mod gps;
use gps::*;

mod image;
use image::*;

mod rf95;
use rf95::*;

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
    
    println!("{}N, {}W", gps.decimal_latitude(), gps.decimal_longitude());

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

    lora.set_frequency(868.5);
    lora.set_tx_power(5);

    println!("Sending...");
    lora.send("$$TELEMETRY TEST".as_bytes());
    lora.wait_packet_sent();

    std::process::exit(0);
}
