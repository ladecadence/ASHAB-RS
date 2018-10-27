pub extern crate i2cdev;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std::thread;
use std::time::Duration;

// MS5607 I2C commands
const MS5607_CMD_RESET: u8      = 0x1E;    // reset
const MS5607_CMD_ADC_READ: u8   = 0x00;    // read sequence
const MS5607_CMD_ADC_CONV: u8   = 0x40;    // start conversion
const MS5607_CMD_ADC_D1: u8     = 0x00;    // read ADC 1
const MS5607_CMD_ADC_D2: u8     = 0x10;    // read ADC 2
const MS5607_CMD_ADC_256: u8    = 0x00;    // ADC oversampling ratio to 256
const MS5607_CMD_ADC_512: u8    = 0x02;    // ADC oversampling ratio to 512
const MS5607_CMD_ADC_1024: u8   = 0x04;    // ADC oversampling ratio to 1024
const MS5607_CMD_ADC_2048: u8   = 0x06;    // ADC oversampling ratio to 2048
const MS5607_CMD_ADC_4096: u8   = 0x08;    // ADC oversampling ratio to 4096
const MS5607_CMD_PROM_RD: u8    = 0xA0;    // readout of PROM registers

struct Ms5607 {
    pub bus: LinuxI2CDevice,
    pub addr: u8,
    pub prom: [u16, 7],
}

impl Ms5607 {
    pub new(b: u8, a: u8) -> Ms5607 {
        Ms5607 {
            bus: LinuxI2CDevice::new(format!("/dev/i2c-{}", b), a)?,
            addr: a,
            buf: [0,0,0,0,0,0,0],
        }
    }

    pub read_prom(&mut self) -> Result<(), &'static str> {
        self.bus.smbus_write_byte_data(0x00, MS5607_CMD_RESET)?;
        thread::sleep(Duration::from_millis(30));
        for i in 0..7 {
            self.prom[i] = 0x0000;
            self.prom[i] = self.bus.smbus_read_word_data();
        }

    }
}

