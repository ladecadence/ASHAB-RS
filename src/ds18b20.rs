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

// Reads temperature data from DS18B20 sensors in the 1-Wire bus.

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[allow(dead_code)]
pub struct DS18B20 {
    pub device: String,
    pub temp: f32,
}

impl DS18B20 {
    pub fn new(dev: &str) -> Self {
        Self {
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
