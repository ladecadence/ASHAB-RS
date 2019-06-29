extern crate spidev;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

#[allow(dead_code)]
pub struct Mcp3002 {
    pub csel: u8,
    pub spidev: Spidev,
}

impl Mcp3002 {
    pub fn new(cs: u8, ch: u8) -> Mcp3002 {
        Mcp3002 {
            csel: cs,
            spidev: Spidev::open(
                String::from("/dev/spidev") + &ch.to_string() + &"." + &cs.to_string(),
            )
            .unwrap(),
        }
    }

    pub fn init(&mut self) {
        // configure SPI
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(488000)
            .mode(SPI_MODE_0)
            .build();
        self.spidev.configure(&options).unwrap();
    }

    pub fn read(&mut self, adc_number: u8) -> Result<u32, &'static str> {
        if adc_number > 1 {
            return Err("Wrong adc channel");
        }

        // Start bit, single channel read
        let mut command = 0b11010000;
        command |= adc_number << 5;

        let tx_buf = [command, 0x00, 0x00];
        let mut rx_buf = [0_u8; 3];

        {
            let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
            self.spidev.transfer(&mut transfer).unwrap();
        }

        let mut result: u32 = (rx_buf[0] as u32 & 0x01) << 9;
        result |= (rx_buf[1] as u32 & 0xff) << 1;
        result |= (rx_buf[2] as u32 & 0x80) >> 7;

        return Ok(result & 0x3ff);
    }
}
