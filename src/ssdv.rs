use std::process::Command;

static SSDV_PROGRAM: &'static str = "ssdv";

pub struct SSDV {
    pub image_file: String,
    pub id: String,
    pub count: u8,
    filename: String,
    path: String,
    pub binaryname: String,
}

impl SSDV {
    pub fn new (img: String, p: String, b: String, i: String, cnt: u8) -> SSDV {
        SSDV {
            image_file : img.clone(),
            id : i,
	    path: p.clone(),
            count : cnt,
            filename : img.clone(), 
            binaryname: p.clone() + &b + ".bin",
        }
    }

    pub fn encode(&mut self) -> Result<(), &'static str> {
        let status = Command::new(SSDV_PROGRAM)
            .arg("-e")
            .arg("-c")
            .arg(&self.id)
            .arg("-i")
            .arg(&format!("{}", self.count))
            .arg(&self.filename)
            .arg(&self.binaryname)
            .status();
        let exit_code: i32;
        match status {
            Ok(s) => exit_code = s.code().unwrap(),
            Err(_e) => return Err("ssdv failed"),
        }

        // ssdv worked, return Ok
        if exit_code == 0 {
            return Ok(());
        }

        // exit code not 0
        Err("ssdv failed")

    }

}

