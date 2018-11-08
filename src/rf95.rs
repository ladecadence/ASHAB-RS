// (C) 2016 David Pello Gonzalez for ASHAB
// Based on code from the RadioHead Library:
// http://www.airspayce.com/mikem/arduino/RadioHead/
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

#![allow (dead_code)]
extern crate spidev;
extern crate sysfs_gpio;

use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};
use std::{thread, time};
use sysfs_gpio::{Direction, Pin};

const FXOSC: f32 = 32000000.0;
const FSTEP: f32 = FXOSC / 524288.0;

// Register names (LoRa Mode, from table 85)
const REG_00_FIFO: u8 = 0x00;
const REG_01_OP_MODE: u8 = 0x01;
const REG_02_RESERVED: u8 = 0x02;
const REG_03_RESERVED: u8 = 0x03;
const REG_04_RESERVED: u8 = 0x04;
const REG_05_RESERVED: u8 = 0x05;
const REG_06_FRF_MSB: u8 = 0x06;
const REG_07_FRF_MID: u8 = 0x07;
const REG_08_FRF_LSB: u8 = 0x08;
const REG_09_PA_CONFIG: u8 = 0x09;
const REG_0A_PA_RAMP: u8 = 0x0a;
const REG_0B_OCP: u8 = 0x0b;
const REG_0C_LNA: u8 = 0x0c;
const REG_0D_FIFO_ADDR_PTR: u8 = 0x0d;
const REG_0E_FIFO_TX_BASE_ADDR: u8 = 0x0e;
const REG_0F_FIFO_RX_BASE_ADDR: u8 = 0x0f;
const REG_10_FIFO_RX_CURRENT_ADDR: u8 = 0x10;
const REG_11_IRQ_FLAGS_MASK: u8 = 0x11;
const REG_12_IRQ_FLAGS: u8 = 0x12;
const REG_13_RX_NB_BYTES: u8 = 0x13;
const REG_14_RX_HEADER_CNT_VALUE_MSB: u8 = 0x14;
const REG_15_RX_HEADER_CNT_VALUE_LSB: u8 = 0x15;
const REG_16_RX_PACKET_CNT_VALUE_MSB: u8 = 0x16;
const REG_17_RX_PACKET_CNT_VALUE_LSB: u8 = 0x17;
const REG_18_MODEM_STAT: u8 = 0x18;
const REG_19_PKT_SNR_VALUE: u8 = 0x19;
const REG_1A_PKT_RSSI_VALUE: u8 = 0x1a;
const REG_1B_RSSI_VALUE: u8 = 0x1b;
const REG_1C_HOP_CHANNEL: u8 = 0x1c;
const REG_1D_MODEM_CONFIG1: u8 = 0x1d;
const REG_1E_MODEM_CONFIG2: u8 = 0x1e;
const REG_1F_SYMB_TIMEOUT_LSB: u8 = 0x1f;
const REG_20_PREAMBLE_MSB: u8 = 0x20;
const REG_21_PREAMBLE_LSB: u8 = 0x21;
const REG_22_PAYLOAD_LENGTH: u8 = 0x22;
const REG_23_MAX_PAYLOAD_LENGTH: u8 = 0x23;
const REG_24_HOP_PERIOD: u8 = 0x24;
const REG_25_FIFO_RX_BYTE_ADDR: u8 = 0x25;
const REG_26_MODEM_CONFIG3: u8 = 0x26;
const REG_28_FREQ_ERROR: u8 = 0x28;
const REG_31_DETECT_OPT: u8 = 0x31;
const REG_37_DETECTION_THRESHOLD: u8 = 0x37;

const REG_40_DIO_MAPPING1: u8 = 0x40;
const REG_41_DIO_MAPPING2: u8 = 0x41;
const REG_42_VERSION: u8 = 0x42;

const REG_4B_TCXO: u8 = 0x4b;
const REG_4D_PA_DAC: u8 = 0x4d;
const REG_5B_FORMER_TEMP: u8 = 0x5b;
const REG_61_AGC_REF: u8 = 0x61;
const REG_62_AGC_THRESH1: u8 = 0x62;
const REG_63_AGC_THRESH2: u8 = 0x63;
const REG_64_AGC_THRESH3: u8 = 0x64;

// REG_01_OP_MODE                             0x01;
const LONG_RANGE_MODE: u8 = 0x80;
const ACCESS_SHARED_REG: u8 = 0x40;
const MODE: u8 = 0x07;
const MODE_SLEEP: u8 = 0x00;
const MODE_STDBY: u8 = 0x01;
const MODE_FSTX: u8 = 0x02;
const MODE_TX: u8 = 0x03;
const MODE_FSRX: u8 = 0x04;
const MODE_RXCONTINUOUS: u8 = 0x05;
const MODE_RXSINGLE: u8 = 0x06;
const MODE_CAD: u8 = 0x07;

// REG_09_PA_CONFIG                           0x09;
const PA_SELECT: u8 = 0x80;
const MAX_POWER: u8 = 0x70;
const OUTPUT_POWER: u8 = 0x0f;

// REG_0A_PA_RAMP                             0x0a;
const LOW_PN_TX_PLL_OFF: u8 = 0x10;
const PA_RAMP: u8 = 0x0f;
const PA_RAMP_3_4MS: u8 = 0x00;
const PA_RAMP_2MS: u8 = 0x01;
const PA_RAMP_1MS: u8 = 0x02;
const PA_RAMP_500US: u8 = 0x03;
const PA_RAMP_250US: u8 = 0x0;
const PA_RAMP_125US: u8 = 0x05;
const PA_RAMP_100US: u8 = 0x06;
const PA_RAMP_62US: u8 = 0x07;
const PA_RAMP_50US: u8 = 0x08;
const PA_RAMP_40US: u8 = 0x09;
const PA_RAMP_31US: u8 = 0x0a;
const PA_RAMP_25US: u8 = 0x0b;
const PA_RAMP_20US: u8 = 0x0c;
const PA_RAMP_15US: u8 = 0x0d;
const PA_RAMP_12US: u8 = 0x0e;
const PA_RAMP_10US: u8 = 0x0f;

// REG_0B_OCP                                 0x0b;
const OCP_ON: u8 = 0x20;
const OCP_TRIM: u8 = 0x1f;

// REG_0C_LNA                                 0x0c;
const LNA_GAIN: u8 = 0xe0;
const LNA_BOOST: u8 = 0x03;
const LNA_BOOST_DEFAULT: u8 = 0x00;
const LNA_BOOST_150PC: u8 = 0x11;

// REG_11_IRQ_FLAGS_MASK                      0x11;
const RX_TIMEOUT_MASK: u8 = 0x80;
const RX_DONE_MASK: u8 = 0x40;
const PAYLOAD_CRC_ERROR_MASK: u8 = 0x20;
const VALID_HEADER_MASK: u8 = 0x10;
const TX_DONE_MASK: u8 = 0x08;
const CAD_DONE_MASK: u8 = 0x04;
const FHSS_CHANGE_CHANNEL_MASK: u8 = 0x02;
const CAD_DETECTED_MASK: u8 = 0x01;

// REG_12_IRQ_FLAGS                           0x12;
const RX_TIMEOUT: u8 = 0x80;
const RX_DONE: u8 = 0x40;
const PAYLOAD_CRC_ERROR: u8 = 0x20;
const VALID_HEADER: u8 = 0x10;
const TX_DONE: u8 = 0x08;
const CAD_DONE: u8 = 0x04;
const FHSS_CHANGE_CHANNEL: u8 = 0x02;
const CAD_DETECTED: u8 = 0x01;

// REG_18_MODEM_STAT                          0x18;
const RX_CODING_RATE: u8 = 0xe0;
const MODEM_STATUS_CLEAR: u8 = 0x10;
const MODEM_STATUS_HEADER_INFO_VALID: u8 = 0x08;
const MODEM_STATUS_RX_ONGOING: u8 = 0x04;
const MODEM_STATUS_SIGNAL_SYNCHRONIZED: u8 = 0x02;
const MODEM_STATUS_SIGNAL_DETECTED: u8 = 0x01;

// REG_1C_HOP_CHANNEL                         0x1c;
const PLL_TIMEOUT: u8 = 0x80;
const RX_PAYLOAD_CRC_IS_ON: u8 = 0x40;
const FHSS_PRESENT_CHANNEL: u8 = 0x3f;

// REG_1D_MODEM_CONFIG1                       0x1d;
const BW_7K8HZ: u8 = 0x00;
const BW_10K4HZ: u8 = 0x10;
const BW_15K6HZ: u8 = 0x20;
const BW_20K8HZ: u8 = 0x30;
const BW_31K25HZ: u8 = 0x40;
const BW_41K7HZ: u8 = 0x50;
const BW_62K5HZ: u8 = 0x60;
const BW_125KHZ: u8 = 0x70;
const BW_250KHZ: u8 = 0x80;
const BW_500KHZ: u8 = 0x90;

const CODING_RATE_4_5: u8 = 0x02;
const CODING_RATE_4_6: u8 = 0x04;
const CODING_RATE_4_7: u8 = 0x06;
const CODING_RATE_4_8: u8 = 0x08;

const IMPLICIT_HEADER_MODE_ON: u8 = 0x00;
const IMPLICIT_HEADER_MODE_OFF: u8 = 0x01;

// REG_1E_MODEM_CONFIG2                       0x1e;
const SPREADING_FACTOR_64CPS: u8 = 0x60;
const SPREADING_FACTOR_128CPS: u8 = 0x70;
const SPREADING_FACTOR_256CPS: u8 = 0x80;
const SPREADING_FACTOR_512CPS: u8 = 0x90;
const SPREADING_FACTOR_1024CPS: u8 = 0xa0;
const SPREADING_FACTOR_2048CPS: u8 = 0xb0;
const SPREADING_FACTOR_4096CPS: u8 = 0xc0;
const TX_CONTINUOUS_MODE_ON: u8 = 0x08;
const TX_CONTINUOUS_MODE_OFF: u8 = 0x00;
const RX_PAYLOAD_CRC_ON: u8 = 0x02;
const RX_PAYLOAD_CRC_OFF: u8 = 0x00;
const SYM_TIMEOUT_MSB: u8 = 0x03;

// REG_26_MODEM_CONFIG3;
const AGC_AUTO_ON: u8 = 0x04;
const AGC_AUTO_OFF: u8 = 0x00;

// REG_4D_PA_DAC                              0x4d;
const PA_DAC_DISABLE: u8 = 0x04;
const PA_DAC_ENABLE: u8 = 0x07;

const MAX_MESSAGE_LEN: u8 = 255;

// default params;
const BW125_CR45_SF128 : (u8, u8, u8) =  (0x72, 0x74, 0x00);
const BW500_CR45_SF128 : (u8, u8, u8) =  (0x92, 0x74, 0x00);
const BW31_25_CR48_SF512 : (u8, u8, u8) =  (0x48, 0x94, 0x00);
const BW125_CR48_SF4096 : (u8, u8, u8) =  (0x78, 0xc4, 0x00);

// SPI;
const SPI_WRITE_MASK: u8 = 0x80;
const SPI_READ_MASK: u8 = 0x7F;

// Modes;
const RADIO_MODE_INITIALISING: u8 = 0;
const RADIO_MODE_SLEEP: u8 = 1;
const RADIO_MODE_IDLE: u8 = 2;
const RADIO_MODE_TX: u8 = 3;
const RADIO_MODE_RX: u8 = 4;
const RADIO_MODE_CAD: u8 = 5;


#[allow(dead_code)]
pub struct RF95 {
    mode: u8,
    buf: [u8; 256],
    buflen: u8,
    last_rssi: i16,
    rx_bad: u16,
    rx_good: u16,
    tx_good: u16,
    rx_buf_valid: bool,
    pub spidev: Spidev,
    pub channel: u8,
    pub int_pin_number: u8,
    int_pin: Pin,
    use_int: bool,
    int_thread: thread::Builder,
    cad: u8,
}

impl RF95 {
    pub fn new(ch: u8, int: u8, use_i: bool) -> RF95 { 
        RF95 {
            mode : RADIO_MODE_INITIALISING,
            buf : [0; 256],
            buflen : 0,
            last_rssi: -99,
            rx_bad: 0,
            rx_good: 0,
            tx_good: 0,
            rx_buf_valid: false,
            channel: ch,
            int_pin_number: int,
            spidev: Spidev::open(String::from("/dev/spidev0.") 
                                 + &ch.to_string()).unwrap(),
                                 int_pin: Pin::new(int as u64),
                                 use_int: use_i,
                                 int_thread: thread::Builder::new()
                                     .name("rf95_int".into()),
                                 cad: 0,
        }
    }

    // write one byte of data to register addr
    pub fn spi_write(&mut self, reg: u8, byte: u8) {
        self.spidev.write(&[reg | SPI_WRITE_MASK, byte]).unwrap();
    }

    // read one byte of data from register addr
    pub fn spi_read(&mut self, reg: u8) -> u8 {
        let mut rx = [0_u8, 2];
        let tx: [u8; 2] = [reg, 0];
        { 
            let mut transfer = SpidevTransfer::read_write(&tx, &mut rx);
            self.spidev.transfer(&mut transfer).unwrap();
        }

        rx[1]
    }

    // write a slice (array) of data to register addr
    pub fn spi_write_data(&mut self, reg: u8, data: &[u8]) {
        // bounds
        if data.len() > MAX_MESSAGE_LEN as usize {
            return;
        }

        // fill tx buf
        let mut tx: Vec<u8> = Vec::new();
        tx.push(reg | SPI_WRITE_MASK);

        tx.extend(data.iter().cloned());

        self.spidev.write(&tx).unwrap();
    }

    pub fn spi_read_data(&mut self, reg: u8, len: u8) -> [u8; 256] {
        let mut data = [0_u8; 256];
        for i in 0..len {
            data[i as usize] = self.spi_read(reg + i);
        }

        return data;
    }

    // configure SPI bus and RF95 LoRa default mode 
    pub fn init(&mut self) -> Result<(), &'static str> {

        // configure SPI and initialize RF95
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(5000)
            .mode(SPI_MODE_0)
            .build();
        self.spidev.configure(&options).unwrap();

        // set LoRa mode
        self.spi_write(REG_01_OP_MODE, MODE_SLEEP | LONG_RANGE_MODE);

        thread::sleep(time::Duration::from_millis(10));

        // check if we are set
        if self.spi_read(REG_01_OP_MODE) != (MODE_SLEEP | LONG_RANGE_MODE) {
            return Err("Lora not configured");
        }

        // set up FIFO
        self.spi_write(REG_0E_FIFO_TX_BASE_ADDR, 0);
        self.spi_write(REG_0F_FIFO_RX_BASE_ADDR, 0);

        // default mode
        self.set_mode_idle();

        self.set_modem_config(BW125_CR45_SF128);
        self.set_preamble_length(8);

        // setup gpio
        if self.use_int {
            self.int_pin.export().unwrap();
            self.int_pin.set_direction(Direction::In).unwrap();
        }

        Ok(())
    }

    pub fn set_frequency(&mut self, freq: f32) {
        let freq_value: i32 = ((freq * 1000000.0) / FSTEP ) as i32;

        self.spi_write(REG_06_FRF_MSB, ((freq_value>>16)&0xff) as u8);
        self.spi_write(REG_07_FRF_MID, ((freq_value>>8)&0xff) as u8);
        self.spi_write(REG_08_FRF_LSB, ((freq_value)&0xff) as u8);
    }

    pub fn set_mode_idle(&mut self) {
        if self.mode != RADIO_MODE_IDLE {
            self.spi_write(REG_01_OP_MODE, MODE_STDBY);
            self.mode = RADIO_MODE_IDLE;
        }
    }

    pub fn set_mode_sleep(&mut self) {
        if self.mode != RADIO_MODE_SLEEP {
            self.spi_write(REG_01_OP_MODE, MODE_SLEEP);
            self.mode = RADIO_MODE_SLEEP;
        }
    }

    pub fn set_mode_rx(&mut self) {
        if self.mode != RADIO_MODE_RX {
            self.spi_write(REG_01_OP_MODE, MODE_RXCONTINUOUS);
            self.spi_write(REG_40_DIO_MAPPING1, 0x00u8);
            self.mode = RADIO_MODE_RX;
        }
    }

    pub fn set_mode_tx(&mut self) {
        if self.mode != RADIO_MODE_TX {
            self.spi_write(REG_01_OP_MODE, MODE_TX);
            self.spi_write(REG_40_DIO_MAPPING1, 0x40u8);
            self.mode = RADIO_MODE_TX;
        }
    }

    pub fn set_tx_power(&mut self, p: u8) {
        let mut power = p;

        // bounds
        if power > 23 {
            power = 23;
        }

        if power < 5 {
            power = 5;
        }

        // A_DAC_ENABLE actually adds about 3dBm to all
        // power levels. We will us it for 21, 22 and 23dBm

        if power>20 {
            self.spi_write(REG_4D_PA_DAC, PA_DAC_ENABLE);
            power = power - 3;
        }
        else {
            self.spi_write(REG_4D_PA_DAC, PA_DAC_DISABLE);
        }

        // write it
        self.spi_write(REG_09_PA_CONFIG, PA_SELECT | (power-5));
    }

    // set mode from default modes
    pub fn set_modem_config(&mut self, mode: (u8, u8, u8)) {
        self.spi_write(REG_1D_MODEM_CONFIG1, mode.0);
        self.spi_write(REG_1E_MODEM_CONFIG2, mode.1);
        self.spi_write(REG_26_MODEM_CONFIG3, mode.2);
    }

    pub fn set_modem_config_custom(&mut self, bandwidth: u8, 
                                   coding_rate: u8, 
                                   implicit_header: u8, 
                                   spreading_factor: u8, 
                                   crc: u8,
                                   continuous_tx: u8, 
                                   timeout: u8, 
                                   agc_auto: u8) {

        self.spi_write(REG_1D_MODEM_CONFIG1, 
                       bandwidth | coding_rate | implicit_header);
        self.spi_write(REG_1E_MODEM_CONFIG2,
                       spreading_factor | continuous_tx | crc | timeout);
        self.spi_write(REG_26_MODEM_CONFIG3, agc_auto);
    }


    pub fn set_preamble_length(&mut self, len: u16) {
        self.spi_write(REG_20_PREAMBLE_MSB, (len >> 8) as u8);
        self.spi_write(REG_21_PREAMBLE_LSB, (len & 0xff) as u8);
    }


    // Send data
    pub fn send(&mut self, data: &[u8]) -> bool{
        if data.len() > MAX_MESSAGE_LEN  as usize {
            return false;
        }

        self.wait_packet_sent();

        self.set_mode_idle();

        // beggining of FIFO
        self.spi_write(REG_0D_FIFO_ADDR_PTR, 0);

        // write data
        self.spi_write_data(REG_00_FIFO, data);
        self.spi_write(REG_22_PAYLOAD_LENGTH, data.len() as u8);

        self.set_mode_tx();

        true
    }

    pub fn wait_packet_sent(&mut self) -> bool {
        if !self.use_int {

            // If we are not currently in transmit mode, 
            // there is no packet to wait for
            if self.mode != RADIO_MODE_TX {
                return false;
            }

            while (self.spi_read(REG_12_IRQ_FLAGS) & TX_DONE ) == 0 {
                thread::sleep(time::Duration::from_millis(10));
            }

            self.tx_good = self.tx_good + 1;

            // clear IRQ flags
            self.spi_write(REG_12_IRQ_FLAGS, 0xff);

            self.set_mode_idle();

            return true;

        } else {

            while self.mode == RADIO_MODE_TX {
                thread::sleep(time::Duration::from_millis(10));
            }

            return true;
        }
    }

    pub fn available(&mut self) -> Result<bool, &'static str> {
        if !self.use_int {
            // read the interrupt register
            let irq_flags = self.spi_read(REG_12_IRQ_FLAGS);

            if (self.mode == RADIO_MODE_RX) && (irq_flags & RX_DONE != 0) {

                // Have received a packet
                let length = self.spi_read(REG_13_RX_NB_BYTES);

                // Reset the fifo read ptr to the beginning of the packet
                let ptr = self.spi_read(REG_10_FIFO_RX_CURRENT_ADDR);
                self.spi_write(REG_0D_FIFO_ADDR_PTR, ptr);
                self.buf = self.spi_read_data(REG_00_FIFO, length);
                self.buflen = length;
                // clear IRQ flags
                self.spi_write(REG_12_IRQ_FLAGS, 0xff);

                // Remember the RSSI of this packet
                // this is according to the doc, but is it really correct?
                // weakest receiveable signals are reported RSSI at about -66
                self.last_rssi = (self.spi_read(REG_1A_PKT_RSSI_VALUE) as i16)
                    - 137;

                // We have received a message.
                // validateRxBuf();  TO BE IMPLEMENTED
                self.rx_good = self.rx_good + 1;
                self.rx_buf_valid = true;
                if self.rx_buf_valid {
                    self.set_mode_idle();
                }
            } else if (self.mode == RADIO_MODE_CAD) 
                && (irq_flags & CAD_DONE != 0) {

                    self.cad = irq_flags & CAD_DETECTED;
                    self.set_mode_idle();
                }

            self.spi_write(REG_12_IRQ_FLAGS, 0xff); // Clear all IRQ flags

            if self.mode == RADIO_MODE_TX {
                return Err("Radio in TX mode");
            }

            self.set_mode_rx();
            return Ok(self.rx_buf_valid);
        }
        else {
            return Ok(false);
        }
    }

    pub fn clear_rx_buf(&mut self) {
        self.rx_buf_valid = false;
        self.buflen = 0;
    }



}

