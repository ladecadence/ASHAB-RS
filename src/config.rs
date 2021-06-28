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

// This object reads mission configuration from an INI file as defined
// in the documentation (see README.md)

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub id: String,
    pub subid: String,
    pub msg: String,
    pub separator: String,
    pub packet_repeat: u32,
    pub packet_delay: u32,

    pub batt_enable_pin: u8,
    pub led_pin: u8,
    pub pwr_pin: u8,

    pub gps_serial_port: String,
    pub gps_speed: u32,

    pub lora_cs: u8,
    pub lora_int_pin: u8,
    pub lora_freq: f32,
    pub lora_low_pwr: u8,
    pub lora_high_pwr: u8,

    pub adc_cs: u8,
    pub adc_vbatt: u8,
    pub adc_v_divider: f32,
    pub adc_v_mult: f32,

    pub temp_internal_addr: String,
    pub temp_external_addr: String,

    pub baro_i2c_bus: u8,
    pub baro_addr: u16,

    pub path_main_dir: String,
    pub path_images_dir: String,
    pub path_log_prefix: String,

    pub ssdv_size: String,
    pub ssdv_name: String,
}


impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            subid: "".to_string(),
            msg: "".to_string(),
            separator: "".to_string(),
            packet_repeat: 0,
            packet_delay: 0,

            batt_enable_pin: 0,
            led_pin: 0,
            pwr_pin: 0,

            gps_serial_port: "".to_string(),
            gps_speed: 0,

            lora_cs: 0,
            lora_int_pin: 0,
            lora_freq: 0.0,
            lora_low_pwr: 0,
            lora_high_pwr: 0,

            adc_cs: 0,
            adc_vbatt: 0,
            adc_v_divider: 0.0,
            adc_v_mult: 0.0,

            temp_internal_addr: "".to_string(),
            temp_external_addr: "".to_string(),

            baro_i2c_bus: 0,
            baro_addr: 0,

            path_main_dir: "".to_string(),
            path_images_dir: "".to_string(),
            path_log_prefix: "".to_string(),

            ssdv_size: "".to_string(),
            ssdv_name: "".to_string(),

        }
    }
}

