extern crate ini;
use ini::Ini;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigErrorType {
    IO,
    Section,
    Parameter,
}

#[derive(Debug)]
pub struct ConfigError {
    pub error_type: ConfigErrorType,
    pub info: &'static str,
}

impl ConfigError {
    pub fn new(e: ConfigErrorType, s: &'static str) -> ConfigError {
        ConfigError {
            error_type: e,
            info: s,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub initialized: bool,
    pub file: String,

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
    pub path_log: String,

    pub ssdv_size: String,
    pub ssdv_name: String,
}

impl Config {
    pub fn new(file: &str) -> Config {
        Config {
            initialized: false,
            file: file.to_string(),

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
            path_log: "".to_string(),

            ssdv_size: "".to_string(),
            ssdv_name: "".to_string(),
        }
    }

    pub fn open(&mut self) -> Result<(), ConfigError> {
        // open config file
        let conf = match Ini::load_from_file(&self.file) {
            Ok(c) => c,
            Err(_e) => {
                return Err(ConfigError::new(
                    ConfigErrorType::IO,
                    "Can't open config file",
                ))
            }
        };

        // get mission section
        let section_mission = match conf.section(Some("mission".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section mission not found",
                ))
            }
        };

        self.id = match section_mission.get("id") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission id not found",
                ))
            }
        };

        self.subid = match section_mission.get("subid") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission subid not found",
                ))
            }
        };
        self.msg = match section_mission.get("msg") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission msg not found",
                ))
            }
        };
        self.separator = match section_mission.get("separator") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission separator not found",
                ))
            }
        };
        self.packet_repeat = match section_mission.get("packet_repeat") {
            Some(p) => p.parse::<u32>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission packet_repeat not found",
                ))
            }
        };
        self.packet_delay = match section_mission.get("packet_delay") {
            Some(p) => p.parse::<u32>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter mission packet_delay not found",
                ))
            }
        };

        // get gpio section
        let section_gpio = match conf.section(Some("gpio".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section gpio not found",
                ))
            }
        };

        self.batt_enable_pin = match section_gpio.get("batt_enable_pin") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter gpio batt_enable_pin not found",
                ))
            }
        };
        self.led_pin = match section_gpio.get("led_pin") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter gpio led_pin not found",
                ))
            }
        };
        self.pwr_pin = match section_gpio.get("pwr_pin") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter gpio pwr_pin not found",
                ))
            }
        };

        // get gps section
        let section_gps = match conf.section(Some("gps".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section gps not found",
                ))
            }
        };

        self.gps_serial_port = match section_gps.get("serial_port") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter serial_port not found",
                ))
            }
        };
        self.gps_speed = match section_gps.get("speed") {
            Some(p) => p.parse::<u32>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter speed not found",
                ))
            }
        };

        // get lora section
        let section_lora = match conf.section(Some("lora".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section lora not found",
                ))
            }
        };

        self.lora_cs = match section_lora.get("cs") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter lora cs not found",
                ))
            }
        };
        self.lora_int_pin = match section_lora.get("int_pin") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter lora cs not found",
                ))
            }
        };
        self.lora_freq = match section_lora.get("freq") {
            Some(p) => p.parse::<f32>().unwrap_or(0.0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter lora int_pin not found",
                ))
            }
        };
        self.lora_low_pwr = match section_lora.get("low_pwr") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter lora low_pwr not found",
                ))
            }
        };
        self.lora_high_pwr = match section_lora.get("high_pwr") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter lora high_pwr not found",
                ))
            }
        };

        // get adc section
        let section_adc = match conf.section(Some("adc".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section adc not found",
                ))
            }
        };

        self.adc_cs = match section_adc.get("cs") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter adc cs not found",
                ))
            }
        };
        self.adc_vbatt = match section_adc.get("vbatt") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter adc vbatt not found",
                ))
            }
        };
        self.adc_v_divider = match section_adc.get("v_divider") {
            Some(p) => p.parse::<f32>().unwrap_or(0.0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter adc v_divider not found",
                ))
            }
        };
        self.adc_v_mult = match section_adc.get("v_mult") {
            Some(p) => p.parse::<f32>().unwrap_or(0.0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter adc v_mult not found",
                ))
            }
        };

        // get temp section
        let section_temp = match conf.section(Some("temp".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section temp not found",
                ))
            }
        };

        self.temp_internal_addr = match section_temp.get("internal_addr") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter temp internal_addr not found",
                ))
            }
        };
        self.temp_external_addr = match section_temp.get("external_addr") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter temp external_addr not found",
                ))
            }
        };

        // get baro section
        let section_baro = match conf.section(Some("baro".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section baro not found",
                ))
            }
        };

        self.baro_i2c_bus = match section_baro.get("i2c_bus") {
            Some(p) => p.parse::<u8>().unwrap_or(0),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter baro i2c_bus not found",
                ))
            }
        };
        // convert from hex
        self.baro_addr = match section_baro.get("i2c_addr") {
            Some(s) => u16::from_str_radix(s.trim_start_matches("0x"), 16).unwrap(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter baro i2c_addr not found",
                ))
            }
        };

        // get path section
        let section_path = match conf.section(Some("paths".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section paths not found",
                ))
            }
        };

        self.path_main_dir = match section_path.get("main_dir") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter path main_dir not found",
                ))
            }
        };
        self.path_images_dir = match section_path.get("images_dir") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter path images_dir not found",
                ))
            }
        };
        self.path_log = match section_path.get("log") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter path log not found",
                ))
            }
        };

        // get ssdv section
        let section_ssdv = match conf.section(Some("ssdv".to_owned())) {
            Some(s) => s,
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Section,
                    "Section ssdv not found",
                ))
            }
        };

        self.ssdv_size = match section_ssdv.get("size") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter ssdv size not found",
                ))
            }
        };
        self.ssdv_name = match section_ssdv.get("name") {
            Some(p) => p.to_string(),
            None => {
                return Err(ConfigError::new(
                    ConfigErrorType::Parameter,
                    "Parameter ssdv name not found",
                ))
            }
        };

        self.initialized = true;
        Ok(())
    }
}
