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

// Gets data from an MCP3002 analog to digital converter

extern crate spidev;
use std::io;

use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

#[derive(Debug)]
pub enum Mcp3002ErrorType {
    Open,
    Configure,
    Read,
    Channel,
}

#[derive(Debug)]
pub struct Mcp3002Error {
    pub error_type: Mcp3002ErrorType,
}

impl Mcp3002Error {
    pub fn new(t: Mcp3002ErrorType) -> Self {
        Self { error_type: t }
    }
}


#[allow(dead_code)]
pub struct Mcp3002 {
    pub csel: u8,
    pub spidev: io::Result<Spidev>,
}

impl Mcp3002 {
    pub fn new(cs: u8, ch: u8) -> Self {
        Self {
            csel: cs,
            spidev: Spidev::open(
                String::from("/dev/spidev") + &ch.to_string() + "." + &cs.to_string(),
            ),
        }
    }

    pub fn init(&mut self) -> Result<(), Mcp3002Error> {
        match self.spidev {
            Ok(_) => {},
            Err(_) => {return Err(Mcp3002Error::new(Mcp3002ErrorType::Open))},
        }
        // configure SPI
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(488000)
            .mode(SPI_MODE_0)
            .build();

        if let Ok(port) = &mut self.spidev {
            match port.configure(&options) {
                Ok(_) => {}
                Err(_e) => { return Err(Mcp3002Error::new(Mcp3002ErrorType::Configure)) },
            }
        }
        Ok(())
    }

    pub fn read(&mut self, adc_number: u8) -> Result<u32, Mcp3002Error> {
        if adc_number > 1 {
            return Err(Mcp3002Error::new(Mcp3002ErrorType::Channel));
        }

        // Start bit, single channel read
        let mut command = 0b11010000;
        command |= adc_number << 5;

        let tx_buf = [command, 0x00, 0x00];
        let mut rx_buf = [0_u8; 3];

        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            if let Ok(port) = &self.spidev {
                match port.transfer(&mut transfer) {
                    Ok(_) => {}
                    Err(_e) => { return Err(Mcp3002Error::new(Mcp3002ErrorType::Read)) },
                }
            }
        }

        let mut result: u32 = (rx_buf[0] as u32 & 0x01) << 9;
        result |= (rx_buf[1] as u32 & 0xff) << 1;
        result |= (rx_buf[2] as u32 & 0x80) >> 7;

        Ok(result & 0x3ff)
    }
}
