use std::process::Command;
use std::io::{Error, ErrorKind};
use std::ffi::OsStr;

extern crate chrono;
use chrono::prelude::*;

extern crate image;
use image::{GenericImage, ImageBuffer, imageops};

static STILL_PROGRAM: &'static str = "raspistill";
static RESIZE_PROGRAM: &'static str = "convert";
static MODIFY_PROGRAM: &'static str = "mogrify";

#[allow(dead_code)]
pub struct Picture {
    pub number: u32,
    pub basename: String,
    pub path: String,
    pub filename: String,
    captured: bool,
}

#[allow(dead_code)]
impl Picture {
    pub fn new(num: u32, name: &str, p: &str) -> Picture {
        Picture {
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

    pub fn resize(&mut self, resized_name: String, 
                    new_size: String) -> Result<(), Error> {
        // can't resize non existant picture
        if !self.captured {
            return Err(Error::new(ErrorKind::NotFound, "No picture available"));
        }

        let img = image::open(&self.filename).unwrap();

        img.save(&resized_name).unwrap();
    
        Ok(())
    
    }

    pub fn add_info(&mut self, 
                       file: String, 
                       id: String, 
                       subid: String, 
                       msg: String) -> Result<(), Error> {

        let status = Command::new(MODIFY_PROGRAM)
            .arg("-fill")
            .arg("white")
            .arg("-pointsize")
            .arg("24")
            //.args(&["-draw", &format!("'text 10,40 \"{}{}\" '", &id, &subid)])
            .arg(&OsStr::new(&format!("-draw \"text 10,40 '{}{}'\"", &id, &subid)))
            .arg(&file)
            .status();

        let exit_code: i32;
        match status {
            Ok(s) => exit_code = s.code().unwrap(),
            Err(_e) => {
                return Err(Error::new(
                    ErrorKind::NotFound, "mogrify failed")
                    )
            }
        }

        // ok
        if exit_code == 0 {
            return Ok(());
        } else {
            return Err(Error::new(ErrorKind::NotFound, "Can't modify picture"));
        }

    }
 

}

