#![allow(dead_code)]
extern crate serial;

use std::io::prelude::*;
use std::io::BufReader;
use serial::prelude::*;
use std::time::Duration;

const FIELD_TIME: usize = 1;
const FIELD_LAT: usize = 2;
const FIELD_NS: usize = 3;
const FIELD_LON: usize = 4;
const FIELD_EW: usize = 5;
const FIELD_SATS: usize = 7;
const FIELD_ALT: usize = 9;
const FIELD_SPEED: usize = 7;
const FIELD_HDG: usize = 8;
const FIELD_DATE: usize = 9;

const MIN_SATS: u8 = 4;

#[allow(dead_code)]
pub struct GPS {
    latitude: f32,
    ns: char,
    longitude: f32,
    ew: char,
    altitude: f32,
    sats: u8,
    heading: f32,
    speed: f32,
    line_gga: String,
    line_rmc: String,
    date: String,
    pub port: Result<serial::posix::TTYPort, serial::Error>,
}

#[allow(dead_code)]
impl GPS {
    pub fn new (port_name: &str) -> GPS { 
        GPS {
            latitude : 4331.50,
            ns : 'N',
            longitude : 00536.76,
            ew : 'W',
            altitude : 0.0,
            sats : 0,
            heading : 0.0,
            speed : 0.0,
            line_gga : String::from(""),
            line_rmc : String::from(""),
            date: String::from(""),
            port : serial::open(port_name),
        }
    }

    pub fn config(&mut self, settings: serial::PortSettings) {
        match self.port.as_ref() {
            Err(err) => panic!("Can't open GPS serial port: {}", err),
            Ok(_) => {}
        }
        self.port.as_mut().unwrap().configure(&settings).unwrap();
        self.port.as_mut().unwrap().set_timeout(Duration::from_millis(1000)).unwrap();
    }   

    pub fn update(&mut self) -> bool {
        let mut reader =  BufReader::new(self.port.as_mut().unwrap());


        // Get GGA line
        self.line_gga.clear();
        let mut is_gga: String = self.line_gga.chars().skip(3).take(3).collect();
        while is_gga != "GGA".to_string() {
            reader.read_line(&mut self.line_gga).unwrap();
            is_gga = self.line_gga.chars().skip(3).take(3).collect();
        }

        // and get RMC line
        self.line_rmc.clear();
        let mut is_rmc: String = self.line_rmc.chars().skip(3).take(3).collect();
        while is_rmc != "RMC".to_string() {
            reader.read_line(&mut self.line_rmc).unwrap();
            is_rmc = self.line_rmc.chars().skip(3).take(3).collect();
        }

        // Now parse data
        let gga_data: Vec<&str> = self.line_gga.split(",").collect();
        let rmc_data: Vec<&str> = self.line_rmc.split(",").collect();

        // enough fields?
        if gga_data.len() >= 9 && rmc_data.len() >= 8 {

            // good fix ?
            match gga_data[FIELD_SATS].parse::<u8>()
            {
                Ok(x) => self.sats = x,
                _ => { self.sats = 0; return false }
            }
            if self.sats < MIN_SATS {
                return false;
            }

            // ok parse elements if possible, if not provide default values
            match gga_data[FIELD_LAT].parse::<f32>()
            {
                Ok(x) => self.latitude = x,
                _ => self.latitude = 0.0
            }

            match gga_data[FIELD_NS].chars().nth(0)
            {
                Some(x) => self.ns = x,
                None => self.ns = 'N'
            }

            match gga_data[FIELD_LON].parse::<f32>()
            {
                Ok(x) => self.longitude = x,
                _ => self.longitude = 0.0
            }

            match gga_data[FIELD_EW].chars().nth(0)
            {
                Some(x) => self.ew = x,
                None => self.ew = 'W'
            }


            match gga_data[FIELD_SATS].parse::<u8>()
            {
                Ok(x) => self.sats = x,
                _ => self.sats = 0
            }

            match gga_data[FIELD_ALT].parse::<f32>()
            {
                Ok(x) => self.altitude = x,
                _ => self.altitude = 0.0
            }

            match rmc_data[FIELD_SPEED].parse::<f32>()
            {
                Ok(x) => self.speed = x,
                _ => self.speed = 0.0
            }

            match rmc_data[FIELD_HDG].parse::<f32>()
            {
                Ok(x) => self.heading = x,
                _ => self.heading = 0.0
            }

            self.date = String::from(rmc_data[FIELD_DATE]);

        }
        else {
            return false;
        }

        return true;

    }

    pub fn decimal_latitude(&self) -> f32 {
        let degrees = (self.latitude/100.0).trunc();
        let fraction = (self.latitude - (degrees*100.0)) / 60.0;

        degrees + fraction
    }

    pub fn decimal_longitude(&self) -> f32 {
        let degrees = (self.longitude/100.0).trunc();
        let fraction = (self.longitude - (degrees*100.0)) / 60.0;

        degrees + fraction
    }

}



