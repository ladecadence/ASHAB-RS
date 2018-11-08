use std::process::Command;
use std::io::{Error, ErrorKind};

extern crate chrono;
use chrono::prelude::*;

static STILL_PROGRAM: &'static str = "raspistill";
static RESIZE_PROGRAM: &'static str = "convert";
static MODIFY_PROGRAM: &'static str = "mogrify";

#[allow(dead_code)]
pub struct Image {
    pub number: u32,
    pub basename: String,
    pub path: String,
    pub filename: String,
    captured: bool,
}

#[allow(dead_code)]
impl Image {
    pub fn new(num: u32, name: &str, p: &str) -> Image {
        Image {
            number : num,
            filename : String::from(p) 
                + name.clone() 
                + &num.to_string() 
                + ".jpg",
            basename : String::from(name),
            path : String::from(p),
            captured: false,
        }
    }

    fn update_name(&mut self) {
        // update name with pic number and current time
        self.filename = self.path.clone() + &self.basename
            + "-" + &Utc::now().to_rfc3339().to_string() 
            + "-" + &self.number.to_string() + ".jpg";
    }

    pub fn capture(&mut self) -> Result<(), Error> {

        // update filename
        self.update_name();

        let status = Command::new(STILL_PROGRAM)
            .arg("-st")
            .arg("-t")
            .arg("1000")
            .arg("-o")
            .arg(&self.filename)
            .status();
        let exit_code: i32;
        match status {
            Ok(s) => exit_code = s.code().unwrap(),
            Err(e) => { 
                println!("{}", e); 
                return Err(Error::new(
                        ErrorKind::NotFound, "raspistill failed")
                          ) 
            }
        }

        // if we manage to capture a picture, 
        // increment filename number		
        if exit_code == 0 {
            self.number = self.number + 1;
            self.captured = true;
            return Ok(());
        }

        // exit code was not 0
        Err(Error::new(ErrorKind::NotFound, "Can't take picture"))	
    }

    pub fn resize(&mut self, resized_name: String, new_size: String) -> Result<(), Error> {
        // can't resize non existant picture
        if !self.captured {
            return Err(Error::new(ErrorKind::NotFound, "No picture available"));
        }

        let status = Command::new(RESIZE_PROGRAM)
            .arg(&self.filename)
            .arg("-resize")
            .arg(&new_size)
            .arg(&(self.path.clone() + &resized_name))
            .status();
        let exit_code: i32;
        match status {
            Ok(s) => exit_code = s.code().unwrap(),
            Err(_e) => {
                return Err(Error::new(
                    ErrorKind::NotFound, "convert failed")
                    )
            }
        }

        // ok
        if exit_code == 0 {
            return Ok(());
        }

        // exit code was not 0
        Err(Error::new(ErrorKind::NotFound, "Can't resize picture"))	
    }
 

}

