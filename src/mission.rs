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

// Main mission code.

extern crate chrono;
extern crate image;
extern crate imageproc;
extern crate ini;
extern crate rusttype;
extern crate serial;
extern crate spidev;
extern crate sysfs_gpio;

use std::thread;
use std::process::Command;
use std::time::{Duration, Instant};
use sysfs_gpio::{Direction, Pin};

// own uses
mod gps;
use gps::*;

mod picture;
use picture::*;

mod rf95;
use rf95::*;

mod ds18b20;
use ds18b20::*;

mod config;
use config::*;

mod log;
use log::*;

mod led;
use led::*;

mod mcp3002;
use mcp3002::*;

mod ms5607;
use ms5607::*;

mod telemetry;
use telemetry::*;

mod ssdv;
use ssdv::*;

// CONFIGURATION
/////////////////

const CONFIG_FILE: &'static str = "/home/pi/nsx.cfg";

// MISSION STRUCT
//////////////////

struct Mission {
    log: Log,
    datalog: Log,
    gps: GPS,
    led: LED,
    mcp3002: Mcp3002,
    batt_en_pin: Pin,
    baro: Ms5607,
    temp_internal: DS18B20,
    temp_external: DS18B20,
    lora: RF95,
    pwr_pin: Pin,
    pwr_sel: u8,
    telem: Telemetry,
    pic: Picture,
}

impl Mission {
    pub fn new(conf: &Config) -> Mission {
        Mission {
            log: Log::new(),
            datalog: Log:: new(),
            gps: GPS::new(&conf.gps_serial_port, conf.gps_speed),
            led: LED::new(conf.led_pin),
            mcp3002: Mcp3002::new(conf.adc_cs, 0),
            batt_en_pin: Pin::new(conf.batt_enable_pin as u64),
            baro: Ms5607::new(conf.baro_i2c_bus, conf.baro_addr),
            temp_internal: DS18B20::new(&conf.temp_internal_addr),
            temp_external: DS18B20::new(&conf.temp_external_addr),
            lora: RF95::new(conf.lora_cs, conf.lora_int_pin, false),
            pwr_pin: Pin::new(conf.pwr_pin as u64),
            pwr_sel: 0,
            telem: Telemetry::new(conf.id.clone(), conf.msg.clone(), conf.separator.clone()),
            pic: Picture::new(
                0,
                "ssdv",
                &(conf.path_main_dir.clone() + &conf.path_images_dir.clone()),
            ),
        }
    }

    pub fn init(&mut self, conf: &Config) {
        // Log
        self.log.init(&(conf.path_main_dir.clone() + &conf.path_log_prefix));
        self.log.log(LogType::Info, "NSX starting.");

        // datalog
        self.datalog.init(&(conf.path_main_dir.clone() + "datalog_"));

        // GPS
        match self.gps.config() {
            Ok(()) => {}
            Err(_e) => {
                println!("Can't open/configure GPS port");
                std::process::exit(1);
            }
        };

        // Status LED
        match self.led.init() {
            Ok(()) => {}
            Err(e) => println!("{}", e),
        }
        self.led.blink();

        // ADC and battery
        self.mcp3002.init();

        match self.batt_en_pin.export() {
            Ok(()) => {}
            Err(err) => {
                println!("Can't export batt GPIO: {}", err);
                std::process::exit(1);
            }
        }

        match self.batt_en_pin.set_direction(Direction::Out) {
            Ok(()) => {}
            Err(err) => {
                println!("Can't set batt GPIO direction: {}", err);
                std::process::exit(1);
            }
        }
        match self.batt_en_pin.set_value(0) {
            Ok(()) => {}
            Err(err) => {
                println!("Can't set batt GPIO value: {}", err);
                std::process::exit(1);
            }
        }

        // Barometer
        self.baro.read_prom().unwrap();

        // LoRa radio
        match self.lora.init() {
            Ok(()) => println!("LoRa init ok"),
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        }

        self.lora.set_frequency(conf.lora_freq);

        // Power selection
        match self.pwr_pin.export() {
            Ok(()) => {}
            Err(err) => {
                println!("Can't export pwr GPIO: {}", err);
                std::process::exit(1);
            }
        }

        match self.pwr_pin.set_direction(Direction::In) {
            Ok(()) => {}
            Err(err) => {
                println!("Can't set pwr GPIO direction: {}", err);
                std::process::exit(1);
            }
        }

        self.pwr_sel = match self.pwr_pin.get_value() {
            Ok(i) => i,
            Err(err) => {
                println!("Can't get pwr GPIO value: {}", err);
                std::process::exit(1);
            }
        };

        match self.pwr_sel {
            0 => self.lora.set_tx_power(conf.lora_low_pwr),
            1 => self.lora.set_tx_power(conf.lora_high_pwr),
            _ => {}
        }

        self.log.log(LogType::Info, &format!("Power selection: {}", self.pwr_sel));
    }

    pub fn update_telemetry(&mut self, conf: &Config) {
        // Update sensor data
        // GPS
        match self.gps.update() {
            Ok(()) => {
                self.log.log(
                    LogType::Data,
                    &format!(
                        "{}{}, {}{}, Alt: {}m, Sats: {}, Date: {}, Time: {}",
                        self.gps.decimal_latitude(),
                        self.gps.ns,
                        self.gps.decimal_longitude(),
                        self.gps.ew,
                        self.gps.altitude,
                        self.gps.sats,
                        self.gps.date,
                        self.gps.time
                    ),
                );
            }
            Err(e) => {
                match e.error_type {
                    GpsErrorType::Sats => {
                        self.log.log(LogType::Warn, "GPS: No hay suficientes sats")
                    }
                    GpsErrorType::GGA => self.log.log(
                        LogType::Warn,
                        &format!("GPS: Error en la sentencia GGA: {}", self.gps.line_gga),
                    ),
                    GpsErrorType::RMC => self
                        .log
                        .log(LogType::Warn, "GPS: Error en la sentencia RMC"),
                    GpsErrorType::Fix => self.log.log(LogType::Warn, "GPS: Error con el Fix"),
                    GpsErrorType::Parse => self
                        .log
                        .log(LogType::Warn, "GPS: Error parseando los datos"),
                    _ => {}
                };
                self.led.err();
            }
        }

        // Baro
        self.baro.update().unwrap();
        self.log.log(
            LogType::Data,
            &format!("BARO: {}", self.baro.get_pres().unwrap()),
        );

        // Temperatures
        let t_in = match self.temp_internal.read() {
            Ok(t) => {
                self.log.log(LogType::Data, &format!("TIN: {}", t));
                t
            }
            Err(e) => {
                self.log
                    .log(LogType::Warn, &format!("Error reading TIN: {}", e));
                9999.0
            }
        };

        let t_out = match self.temp_external.read() {
            Ok(t) => {
                self.log.log(LogType::Data, &format!("TOUT: {}", t));
                t
            }
            Err(e) => {
                self.log
                    .log(LogType::Warn, &format!("Error reading TOUT: {}", e));
                9999.0
            }
        };

        // Battery, enable reading, read ADC channel and make conversion
        self.batt_en_pin.set_value(1).unwrap();

        // wait 1ms for current to stabilize
        thread::sleep(Duration::from_millis(1));

        let adc_batt = match self.mcp3002.read(conf.adc_vbatt) {
            Ok(n) => {
                self.log.log(LogType::Data, &format!("ADC0: {}", n));
                n
            }
            Err(e) => {
                self.log
                    .log(LogType::Warn, &format!("Error reading ADC: {}", e));
                0
            }
        };

        self.batt_en_pin.set_value(0).unwrap();

        let vbatt: f32 =
            conf.adc_v_mult * conf.adc_v_divider * (adc_batt as f32 * 3.3 / 1023.0);
        self.log.log(LogType::Data, &format!("VBATT: {}", vbatt));

        // Create telemetry packet
        self.telem.update(
            self.gps.latitude,
            self.gps.ns,
            self.gps.longitude,
            self.gps.ew,
            self.gps.altitude,
            self.gps.heading,
            self.gps.speed,
            self.gps.sats,
            vbatt,
            self.baro.get_pres().unwrap(),
            t_in,
            t_out,
        );
    }

    pub fn send_telemetry(&mut self) {
        // Send telemetry
        self.log.log(LogType::Info, "Sending telemetry packet...");
        self.lora.send(self.telem.aprs_string().as_bytes());
        self.lora.wait_packet_sent();
        self.log.log(LogType::Info, "Telemetry packet sent.");
        self.led.blink();
    }

    pub fn send_ssdv(&mut self, conf: &Config) {
        // Take picture
        match self.pic.capture() {
            Ok(()) => self.log.log(
                LogType::Info,
                &format!("Picture shot: {}", self.pic.filename),
            ),
            Err(e) => self
                .log
                .log(LogType::Error, &format!("Error taking picture {:?}", e)),
        };

        // Take SSDV picture
        match self
            .pic
            .capture_small(conf.ssdv_name.clone(), conf.ssdv_size.clone())
        {
            Ok(()) => self.log.log(
                LogType::Info,
                &format!("SSDV picture shot: {}", conf.ssdv_name.clone()),
            ),
            Err(e) => self.log.log(
                LogType::Error,
                &format!("Error taking SSDV picture {:?}", e),
            ),
        };

        // Encode SSDV picture
        match self.pic.add_info(
            conf.path_main_dir.clone() + &conf.path_images_dir.clone() + &conf.ssdv_name.clone(),
            conf.id.clone(),
            conf.subid.clone(),
            conf.msg.clone(),
            format!(
                "{}{}, {}{}, {}m",
                self.gps.decimal_latitude(),
                self.gps.ns,
                self.gps.decimal_longitude(),
                self.gps.ew,
                self.gps.altitude,
            ),
        ) {
            Ok(()) => self.log.log(LogType::Info, "SSDV Image info added."),
            Err(e) => self.log.log(
                LogType::Error,
                &format!("SSDV Image info adding error {:?}", e),
            ),
        };

        let mut ssdv: SSDV = SSDV::new(
            conf.path_main_dir.clone() + &conf.path_images_dir.clone() + &conf.ssdv_name.clone(),
            conf.path_main_dir.clone() + &conf.path_images_dir.clone(),
            conf.ssdv_name.clone(),
            conf.id.clone(),
            self.pic.number,
        );

        match ssdv.encode() {
            Ok(()) => self.log.log(
                LogType::Info,
                &format!(
                    "SSDV Image {}, Packets encoded: {}",
                    ssdv.binaryname, ssdv.packets
                ),
            ),
            Err(e) => self
                .log
                .log(LogType::Error, &format!("Error encoding SSDV: {:?}", e)),
        };

        // Send SSDV
        self.log.log(LogType::Info, "Sending SSDV image...");
        // get time
        let mut last_time = Instant::now();
        for i in 0..ssdv.packets {
            self.lora.send(&ssdv.get_packet(i).unwrap());
            self.lora.wait_packet_sent();

            // check if we need to send telemetry between image packets
            let now = Instant::now();
            if now.duration_since(last_time).as_secs() > conf.packet_delay as u64 {
                self.update_telemetry(&conf);
                self.send_telemetry();
                last_time = Instant::now();
            }


            thread::sleep(Duration::from_millis(10));
        }

        self.log.log(
            LogType::Info,
            &format!("SSDV Image {} packets sent.", ssdv.packets),
        );
    }
}

// MAIN
//////////////////

fn main() {
    // Test configuration file
    let mut config: Config = Config::new(CONFIG_FILE);
    match config.open() {
        Ok(()) => println!("{}", config.id),
        Err(e) => {
            println!("Error: {}", e.info);
            std::process::exit(1);
        }
    };

    // create mission and configure it
    let mut mission: Mission = Mission::new(&config);
    mission.init(&config);

    // Ok, now get time from GPS and update system time
    match mission.gps.update() {
        Ok(()) => {},
        Err(e) => {
            mission.log.log(LogType::Error, &format!("Error updating GPS: {:?}", e));
        }
    }
    match mission.gps.get_time() {
        Ok(time) => {
            let (hour, min, sec) = time;
            let status = Command::new("date")
            	.arg("-u")
                .arg("+%T")
                .arg("-s")
                .arg(&format!("{:02}:{:02}:{:02}", hour, min, sec))
                .status();
            let exit_code: i32;
            match status {
                Ok(s) => {
                    exit_code = s.code().unwrap();
                    mission.log.log(
                        LogType::Info,
                        &format!(
                            "System time set: {:02}:{:02}:{:02} - {:?}",
                            hour, min, sec, exit_code
                        )
                    );
                },
                Err(e) => {
                    mission.log.log(LogType::Error, &format!("Error setting time: {:?}", e));
                },
            }
        },
        Err(e) => {
            mission.log.log(LogType::Error, &format!("Error getting GPS time: {:?}", e));
        },
    }

    ///////// MAIN LOOP /////////
    loop {
        // Telemetry
        for _i in 0..config.packet_repeat {
            // Check for commands

            // Send telemetry
            mission.update_telemetry(&config);
            mission.send_telemetry();
            
            // write datalog
            mission.datalog.log(LogType::Clean, &mission.telem.csv_string());

            // Wait
            thread::sleep(Duration::from_millis(config.packet_delay as u64 * 1000));
        }

        // send SSDV
        mission.send_ssdv(&config);

        // Wait
        thread::sleep(Duration::from_millis(config.packet_delay as u64 * 1000));
    }
}
