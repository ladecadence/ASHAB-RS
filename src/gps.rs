#![allow(dead_code)]
extern crate serial;

use serial::prelude::*;
use serial::BaudRate;
use std::io::prelude::*;
use std::io::BufReader;
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

#[derive(Debug)]
pub enum GpsErrorType {
    Open,
    GGA,
    RMC,
    Sats,
    Fix,
    Parse,
}

#[derive(Debug)]
pub struct GpsError {
    pub error_type: GpsErrorType,
}

impl GpsError {
    pub fn new(t: GpsErrorType) -> GpsError {
        GpsError { error_type: t }
    }
}

#[allow(dead_code)]
pub struct GPS {
    pub latitude: f32,
    pub ns: char,
    pub longitude: f32,
    pub ew: char,
    pub altitude: f32,
    pub sats: u8,
    pub heading: f32,
    pub speed: f32,
    pub line_gga: String,
    pub line_rmc: String,
    pub date: String,
    pub port: Result<serial::posix::TTYPort, serial::Error>,
    settings: serial::PortSettings,
}

#[allow(dead_code)]
impl GPS {
    pub fn new(port_name: &str, port_speed: u32) -> GPS {
        GPS {
            latitude: 4332.94,
            ns: 'N',
            longitude: 00539.78,
            ew: 'W',
            altitude: 0.0,
            sats: 0,
            heading: 0.0,
            speed: 0.0,
            line_gga: String::from(""),
            line_rmc: String::from(""),
            date: String::from(""),
            port: serial::open(port_name),
            settings: serial::PortSettings {
                baud_rate: BaudRate::from_speed(port_speed as usize),
                char_size: serial::Bits8,
                parity: serial::ParityNone,
                stop_bits: serial::Stop1,
                flow_control: serial::FlowNone,
            },
        }
    }

    pub fn config(&mut self) -> Result<(), GpsError> {
        match self.port.as_ref() {
            Err(_e) => return Err(GpsError::new(GpsErrorType::Open)),
            Ok(_) => {}
        }

        self.port
            .as_mut()
            .unwrap()
            .configure(&self.settings)
            .unwrap();
        self.port
            .as_mut()
            .unwrap()
            .set_timeout(Duration::from_millis(1000))
            .unwrap();
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), GpsError> {
        let mut reader = BufReader::new(self.port.as_mut().unwrap());

        // Get GGA line
        self.line_gga.clear();
        let mut is_gga: String = self.line_gga.chars().skip(3).take(3).collect();
        while is_gga != "GGA".to_string() {
            self.line_gga.clear();
            match reader.read_line(&mut self.line_gga) {
                Ok(_) => {}
                Err(e) => {
                    // match utf8 conversion errors
                    match e.kind() {
                        std::io::ErrorKind::InvalidData => {}
                        _ => return Err(GpsError::new(GpsErrorType::GGA)),
                    }
                }
            }
            is_gga = self.line_gga.chars().skip(3).take(3).collect();
        }

        // and get RMC line
        self.line_rmc.clear();
        let mut is_rmc: String = self.line_rmc.chars().skip(3).take(3).collect();
        while is_rmc != "RMC".to_string() {
            self.line_rmc.clear();
            match reader.read_line(&mut self.line_rmc) {
                Ok(_) => {}
                Err(e) => match e.kind() {
                    std::io::ErrorKind::InvalidData => {}
                    _ => return Err(GpsError::new(GpsErrorType::RMC)),
                },
            }
            is_rmc = self.line_rmc.chars().skip(3).take(3).collect();
        }

        // Now parse data
        let gga_data: Vec<&str> = self.line_gga.split(",").collect();
        let rmc_data: Vec<&str> = self.line_rmc.split(",").collect();

        // enough fields?
        if gga_data.len() >= 9 && rmc_data.len() >= 8 {
            // good fix ?
            match gga_data[FIELD_SATS].parse::<u8>() {
                Ok(x) => self.sats = x,
                _ => {
                    self.sats = 0;
                    return Err(GpsError::new(GpsErrorType::Fix));
                }
            }
            if self.sats < MIN_SATS {
                return Err(GpsError::new(GpsErrorType::Sats));
            }

            // ok parse elements if possible, if not provide default values
            self.latitude = gga_data[FIELD_LAT].parse::<f32>().unwrap_or(0.0);

            self.ns = gga_data[FIELD_NS].chars().nth(0).unwrap_or('N');

            self.longitude = gga_data[FIELD_LON].parse::<f32>().unwrap_or(0.0);

            self.ew = gga_data[FIELD_EW].chars().nth(0).unwrap_or('W');

            self.sats = gga_data[FIELD_SATS].parse::<u8>().unwrap_or(0);

            self.altitude = gga_data[FIELD_ALT].parse::<f32>().unwrap_or(0.0);

            self.speed = rmc_data[FIELD_SPEED].parse::<f32>().unwrap_or(0.0);

            self.heading = rmc_data[FIELD_HDG].parse::<f32>().unwrap_or(0.0);

            self.date = String::from(rmc_data[FIELD_DATE]);
        } else {
            return Err(GpsError::new(GpsErrorType::Parse));
        }

        Ok(())
    }

    pub fn decimal_latitude(&self) -> f32 {
        let degrees = (self.latitude / 100.0).trunc();
        let fraction = (self.latitude - (degrees * 100.0)) / 60.0;

        degrees + fraction
    }

    pub fn decimal_longitude(&self) -> f32 {
        let degrees = (self.longitude / 100.0).trunc();
        let fraction = (self.longitude - (degrees * 100.0)) / 60.0;

        degrees + fraction
    }
}
