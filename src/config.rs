extern crate ini;
use ini::Ini;

#[derive(Debug)]
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

    pub gps_serial_port: String,
    pub gps_speed: u32,

    pub lora_cs: u8,
    pub lora_int_pin: u8,
    pub lora_freq: f32,
    pub lora_low_pwr: u32,
    pub lora_high_pwr: u32,

    pub adc_cs: u8,
    pub adc_vbatt: u32,
    pub adc_v_divider: f32,
    pub adc_v_mult: u32,

    pub temp_internal_addr: String,
    pub temp_external_addr: String,

    pub baro_i2c_bus: u8,
    pub baro_addr: u16,

    pub path_main_dir: String,
    pub path_images_dir: String,
    pub path_log: String,

    pub ssdv_size: String,
    pub ssdv_name: String
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
            adc_v_mult: 0,

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
            Err(_e) => return Err(ConfigError::new(
                    ConfigErrorType::IO, 
                    "Can't open config file")
                                 ),
        };
        // get mission section
        let section_mission = match conf.section(Some("mission".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section mission not found")
                              ),
        };

        self.id = section_mission.get("id").unwrap().to_string();
        self.subid = section_mission.get("subid").unwrap().to_string();
        self.msg = section_mission.get("msg").unwrap().to_string();
        self.separator = section_mission.get("separator")
            .unwrap()
            .to_string();
        self.packet_repeat = section_mission.get("packet_repeat")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        self.packet_delay = section_mission.get("packet_delay")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // get gpio section
        let section_gpio = match conf.section(Some("gpio".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section gpio not found")
                              ),
        };

        self.batt_enable_pin = section_gpio.get("batt_enable_pin")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.led_pin = section_gpio.get("led_pin")
            .unwrap()
            .parse::<u8>()
            .unwrap();

        // get gps section
        let section_gps = match conf.section(Some("gps".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section gps not found")
                              ),
        };

        self.gps_serial_port = section_gps.get("serial_port")
            .unwrap()
            .to_string();
        self.gps_speed = section_gps.get("speed")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // get lora section
        let section_lora = match conf.section(Some("lora".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section lora not found")
                              ),
        };

        self.lora_cs = section_lora.get("cs")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.lora_int_pin = section_lora.get("int_pin")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.lora_freq = section_lora.get("freq")
            .unwrap()
            .parse::<f32>()
            .unwrap();
        self.lora_low_pwr = section_lora.get("low_pwr")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        self.lora_high_pwr = section_lora.get("high_pwr")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // get adc section
        let section_adc = match conf.section(Some("adc".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section adc not found")
                              ),
        };

        self.adc_cs = section_adc.get("cs")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.adc_vbatt = section_adc.get("vbatt")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        self.adc_v_divider = section_adc.get("v_divider")
            .unwrap()
            .parse::<f32>()
            .unwrap();
        self.adc_v_mult = section_adc.get("v_mult")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // get temp section
        let section_temp = match conf.section(Some("temp".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section temp not found")
                              ),
        };

        self.temp_internal_addr = section_temp.get("internal_addr")
            .unwrap()
            .to_string();
        self.temp_external_addr = section_temp.get("external_addr")
            .unwrap()
            .to_string();

        // get baro section
        let section_baro = match conf.section(Some("baro".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section baro not found")
                              ),
        };

        self.baro_i2c_bus = section_baro.get("i2c_bus")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        // convert from hex
        self.baro_addr = u16::from_str_radix(section_baro.get("i2c_addr")
                                             .unwrap()
                                             .trim_left_matches("0x"), 16)
            .unwrap();

        // get path section
        let section_path = match conf.section(Some("paths".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section paths not found")
                              ),
        };

        self.path_main_dir = section_path.get("main_dir")
            .unwrap()
            .to_string();
        self.path_images_dir = section_path.get("images_dir")
            .unwrap()
            .to_string();
        self.path_log = section_path.get("log")
            .unwrap()
            .to_string();

        // get ssdv section
        let section_ssdv = match conf.section(Some("ssdv".to_owned())) {
            Some(s) => s,
            None => return Err(ConfigError::new(
                    ConfigErrorType::Section, 
                    "Section ssdv not found")
                              ),
        };

        self.ssdv_size = section_ssdv.get("size").unwrap().to_string();
        self.ssdv_name = section_ssdv.get("name").unwrap().to_string();

        self.initialized = true;
        Ok(())
    }
}

