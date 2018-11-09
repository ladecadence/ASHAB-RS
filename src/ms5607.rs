pub extern crate i2cdev;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, };

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

pub struct Ms5607 {
    pub bus: LinuxI2CDevice,
    pub addr: u16,
    pub prom: [u16; 7],
    temp: i64,
    p: i64,
}

#[allow(dead_code)]
impl Ms5607 {
    pub fn new(b: u8, a: u16) -> Ms5607 {
        Ms5607 {
            bus: LinuxI2CDevice::new(format!("/dev/i2c-{}", b), a).unwrap(),
            addr: a,
            prom: [0,0,0,0,0,0,0],
            temp: 0,
            p: 0,
        }
    }

    pub fn read_prom(&mut self) -> Result<(), &'static str> {
        self.bus.smbus_write_byte_data(0x00, MS5607_CMD_RESET).unwrap();
        thread::sleep(Duration::from_millis(30));
        let mut temp: [u8; 2] = [0, 0];
        for i in 0..7 {
            self.prom[i] = 0x0000;
            self.bus.write(&[MS5607_CMD_PROM_RD+(2*i as u8)]).unwrap();
            self.bus.read(&mut temp).unwrap();
            self.prom[i] = (temp[0] as u16) << 8;
            self.prom[i] = self.prom[i] + temp[1] as u16;

        }
        //println!("Prom: {:?}", self.prom);

        Ok(())
    }

    pub fn read_adc(&mut self, cmd: u8) -> Result<i64, &'static str> {
        // start conversion
        self.bus.smbus_write_byte_data(MS5607_CMD_ADC_CONV+cmd, 0).unwrap();

        // wait for ADC
        match cmd & 0x0f {
            MS5607_CMD_ADC_256 => thread::sleep(Duration::from_millis(1)),
            MS5607_CMD_ADC_512 => thread::sleep(Duration::from_millis(3)),
            MS5607_CMD_ADC_1024 => thread::sleep(Duration::from_millis(4)),
            MS5607_CMD_ADC_2048 => thread::sleep(Duration::from_millis(6)),
            MS5607_CMD_ADC_4096 => thread::sleep(Duration::from_millis(10)),
            _ => thread::sleep(Duration::from_millis(10)),
        }

        // read result bytes and create converted value
        let mut data: [u8; 3] = [0, 0, 0];
        self.bus.write(&[MS5607_CMD_ADC_READ]).unwrap();

        self.bus.read(&mut data).unwrap();

        let value: i64 = ((data[0] as i64) << 16) 
            + ((data[1] as i64) << 8) 
            + data[2] as i64;

        Ok(value)
    }

    pub fn update(&mut self) -> Result<(), &'static str> {
        let d2: i64 = self.read_adc(MS5607_CMD_ADC_D2+MS5607_CMD_ADC_4096)
            .unwrap();
        let d1: i64 = self.read_adc(MS5607_CMD_ADC_D1+MS5607_CMD_ADC_4096)
            .unwrap();

        // calculate 1st order pressure and temperature 
        // (MS5607 1st order algorithm)
        let dt: i64 = d2 - self.prom[5] as i64 * (2_i64.pow(8));
        let mut off: i64  = self.prom[2] as i64 * (2_i64.pow(17))
            + dt * self.prom[4] as i64 /(2_i64.pow(6));
        let mut sens: i64 = self.prom[1] as i64 * (2_i64.pow(16))
            + dt * self.prom[3] as i64 /(2_i64.pow(7));
        self.temp = 2000 + (dt * self.prom[6] as i64 ) / (2_i64.pow(23));
        self.p = ((d1*sens) / (2_i64.pow(21)) - off) / (2_i64.pow(15));

        let mut t2: i64 = 0;
        let mut off2: i64 = 0;
        let mut sens2: i64 = 0;

        // perform higher order corrections
        if self.temp < 2000 {
            t2=dt*dt/(2_i64.pow(31));
            off2=61*(self.temp-2000)*(self.temp-2000)/(2_i64.pow(4));
            sens2=2*(self.temp-2000)*(self.temp-2000);

            if self.temp < -1500 {
                off2=off2+(15*(self.temp+1500)*(self.temp+1500));
                sens2=sens2+(8*(self.temp+1500)*(self.temp+1500));
            }
        }

        self.temp = self.temp-t2;
        off = off - off2;
        sens = sens - sens2;

        self.p=((d1*sens)/(2_i64.pow(21))-off)/(2_i64.pow(15));

        Ok(())
    }

    pub fn get_temp(&mut self) -> Result<f32, &'static str> {
        return Ok(self.temp as f32 / 100.0);
    }

    pub fn get_pres(&mut self) -> Result<f32, &'static str> {
        return Ok(self.p as f32 / 100.0);
    }
}

