use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

#[allow(dead_code)]
pub struct DS18B20 {
    pub device: String,
    pub temp: f32,
}


impl DS18B20 {
    pub fn new (dev: &str) -> DS18B20 {
        DS18B20 {
            device: String::from("/sys/bus/w1/devices/") 
                + String::from(dev).as_str() 
                + String::from("/w1_slave").as_str(),
            temp: 999.99,
        }

    }

    pub fn read(&mut self) -> Result<f32, io::Error> {
        // try to open file or return Err
        let f = match File::open(self.device.as_str()) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut reader = BufReader::new(f);

        let mut buffer = String::new();
        // read second line into buffer
        reader.read_line(&mut buffer).unwrap();
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();

        // ok, we have second line in buffer, parse it
        let data: Vec<&str> = buffer.split(" ").collect();

        self.temp = f32::from_str(&data[9][2..].trim()).unwrap_or(999999.0);
        self.temp = self.temp / 1000.0;

        // return Ok(temp)
        Ok(self.temp)
    }
}


