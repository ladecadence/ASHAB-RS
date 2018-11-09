use std::process::Command;
use std::io::{Error, ErrorKind};

extern crate chrono;
use chrono::prelude::*;

extern crate image;
extern crate imageproc;
use imageproc::drawing::draw_text_mut;
use image::{Rgba,};

extern crate rusttype;
use rusttype::{FontCollection, Scale};

static STILL_PROGRAM: &'static str = "raspistill";

const TEXT_BIG: f32 = 12.0;
const TEXT_SMALL: f32 = 8.0;

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

    pub fn capture_small(&mut self, name: String, res: String) -> Result<(), Error> {

	// get resolution
	let resolution: Vec<&str> = res.split("x").collect();
	        
	let status = Command::new(STILL_PROGRAM)
            .arg("-st")
            .arg("-t")
            .arg("1000")
	    .arg("-w")
	    .arg(resolution[0])
	    .arg("-h")
	    .arg(resolution[1])
            .arg("-o")
            .arg(&(self.path.clone() + &name))
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
        if exit_code == 0 {
            return Ok(());
        }

        // exit code was not 0
        Err(Error::new(ErrorKind::NotFound, "Can't take picture"))	
    }


    pub fn add_info(&mut self, 
                       file: String, 
                       id: String, 
                       subid: String, 
                       msg: String,
                       data: String ) -> Result<(), Error> {

        let datetime = Utc::now().to_rfc3339().to_string();

        let mut image = image::open(&file).unwrap();
        let font = Vec::from(include_bytes!("font.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

        let scale = Scale { x: TEXT_BIG * 2.0, y: TEXT_BIG };
        draw_text_mut(&mut image, 
            Rgba([0u8, 0u8, 0u8, 255u8]), 
            10, 
            20, 
            scale, 
            &font, 
            &format!("{}{}", &id, &subid));
        draw_text_mut(&mut image, 
            Rgba([255u8, 255u8, 255u8, 255u8]), 
            12, 
            22, 
            scale, 
            &font, 
            &format!("{}{}", &id, &subid));
        
        let scale = Scale { x: TEXT_SMALL, y: TEXT_SMALL };
        draw_text_mut(&mut image, 
            Rgba([0u8, 0u8, 0u8, 0u8]), 
            10, 
            45, 
            scale, 
            &font, 
            &format!("{}", &msg));
        draw_text_mut(&mut image, 
            Rgba([255u8, 255u8, 255u8, 255u8]), 
            11, 
            46, 
            scale, 
            &font, 
            &format!("{}", &msg));
        draw_text_mut(&mut image, 
            Rgba([0u8, 0u8, 0u8, 0u8]), 
            10, 
            65, 
            scale, 
            &font, 
            &format!("{}", &datetime));
        draw_text_mut(&mut image, 
            Rgba([255u8, 255u8, 255u8, 255u8]), 
            11, 
            66, 
            scale, 
            &font, 
            &format!("{}", &datetime));
        draw_text_mut(&mut image, 
            Rgba([0u8, 0u8, 0u8, 0u8]), 
            10, 
            80, 
            scale, 
            &font, 
            &format!("{}", &data));
        draw_text_mut(&mut image, 
            Rgba([255u8, 255u8, 255u8, 255u8]), 
            11, 
            81, 
            scale, 
            &font, 
            &format!("{}", &data));
     
        
        image.save(&file).unwrap();

        //return Err(Error::new(ErrorKind::NotFound, "Can't modify picture"));
        Ok(())

    }
 

}

