extern crate serial;
use std::io::prelude::*;

mod gps;
use gps::*;

mod image;
use image::*;

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
    let mut gps: GPS =  GPS::new(GPS_PORT_NAME);
    gps.config(GPS_PORT_SETTINGS);
    
    for i in 0..256 {
        let data = &mut[0];
        data[0] = i as u8;
        gps.port.as_mut().unwrap().write(data).unwrap();
        data[0] = 0;
        gps.port.as_mut().unwrap().read(data).unwrap();
        println!("{}", data[0]);
    }

    println!("{}N, {}W", gps.decimal_latitude(), gps.decimal_longitude());

    let mut img: Image = Image::new(0, "ssdv", "/home/pi");
    if img.capture() {
	println!("Capturada imagen {}", img.filename);
    }
    else {
	println!("Error");
    }

    img.capture();

    std::process::exit(0);
}
