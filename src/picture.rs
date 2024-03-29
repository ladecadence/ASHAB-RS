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

// Uses the raspberry pi camera to take pictures and add mission data
// over them

use std::process::Command;

extern crate chrono;
use chrono::prelude::*;

extern crate image;
extern crate imageproc;
use image::Rgba;
use imageproc::drawing::draw_text_mut;

extern crate rusttype;
use rusttype::{Scale, Font};

// Constants and macros
static STILL_PROGRAM: &'static str = "raspistill";
/*
macro_rules! FONT_FILE {
    () => {
        "TerminusTTF-4.46.0.ttf"
    };
}
*/

const TEXT_BIG: f32 = 20.0;
const TEXT_SMALL: f32 = 16.0;

// Possible errors

#[allow(dead_code)]
#[derive(Debug)]
pub enum PictureErrorType {
    Camera,
    Capture,
    Modify,
    IO,
}

#[derive(Debug)]
pub struct PictureError {
    pub error_type: PictureErrorType,
}

impl PictureError {
    pub fn new(t: PictureErrorType) -> Self {
        Self { error_type: t }
    }
}

#[allow(dead_code)]
pub struct Picture {
    pub number: u8,
    pub basename: String,
    pub path: String,
    pub filename: String,
    captured: bool,
}

#[allow(dead_code)]
impl Picture {
    pub fn new(num: u8, name: &str, p: &str) -> Self {
        Self {
            number: num,
            filename: String::from(p) + name.clone() + &num.to_string() + ".jpg",
            basename: String::from(name),
            path: String::from(p),
            captured: false,
        }
    }

    fn update_name(&mut self) {
        // update name with pic number and current time
        self.filename = self.path.clone()
            + &self.basename
            + "-"
            + &Utc::now().to_rfc3339()
            + "-"
            + &self.number.to_string()
            + ".jpg";
    }

    pub fn capture(&mut self) -> Result<(), PictureError> {
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
                return Err(PictureError::new(PictureErrorType::Camera));
            }
        }

        // if we manage to capture a picture,
        // increment filename number
        if exit_code == 0 {
            if self.number == 255 {
                self.number = 0; 
            } else {
                self.number += 1;
            }
            self.captured = true;
            return Ok(());
        }

        // exit code was not 0
        Err(PictureError::new(PictureErrorType::Capture))
    }

    pub fn capture_small(&mut self, name: String, res: String) -> Result<(), PictureError> {
        // get resolution
        let resolution: Vec<&str> = res.split('x').collect();

        // capture image
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
                return Err(PictureError::new(PictureErrorType::Camera));
            }
        }

        // if we manage to capture a picture,
        if exit_code == 0 {
            return Ok(());
        }

        // exit code was not 0
        Err(PictureError::new(PictureErrorType::Capture))
    }

    // add basic data to pictures to be sent by SSDV
    pub fn add_info(
        &mut self,
        file: String,
        id: String,
        subid: String,
        msg: String,
        data: String,
    ) -> Result<(), PictureError> {
        // get date
        let datetime = Utc::now().to_rfc3339();

        // try to open image
        let mut image = match image::open(&file) {
            Ok(i) => i,
            Err(_e) => return Err(PictureError::new(PictureErrorType::IO)),
        };

        // create font
        /*
        let font = Vec::from(include_bytes!(FONT_FILE!()) as &[u8]);
        let font = match FontCollection::from_bytes(font) {
            Ok(f) => f,
            Err(_e) => return Err(PictureError::new(PictureErrorType::Modify)),
        };
        let font = match font.into_font() {
            Ok(f) => f,
            Err(_e) => return Err(PictureError::new(PictureErrorType::Modify)),
        };
        */
        let font = Vec::from(include_bytes!("TerminusTTF-4.46.0.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        // add data
        let scale = Scale {
            x: TEXT_BIG * 2.0,
            y: TEXT_BIG,
        };

        draw_text_mut(
            &mut image,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            10,
            20,
            scale,
            &font,
            &format!("{}{}", &id, &subid),
        );

        draw_text_mut(
            &mut image,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            12,
            22,
            scale,
            &font,
            &format!("{}{}", &id, &subid),
        );

        let scale = Scale {
            x: TEXT_SMALL,
            y: TEXT_SMALL,
        };

        draw_text_mut(
            &mut image,
            Rgba([0u8, 0u8, 0u8, 0u8]),
            10,
            45,
            scale,
            &font,
            &msg.to_string());

        draw_text_mut(
            &mut image,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            11,
            46,
            scale,
            &font,
            &msg.to_string(),
        );
        draw_text_mut(
            &mut image,
            Rgba([0u8, 0u8, 0u8, 0u8]),
            10,
            65,
            scale,
            &font,
            &datetime.to_string(),
        );
        draw_text_mut(
            &mut image,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            11,
            66,
            scale,
            &font,
            &datetime.to_string(),
        );
        draw_text_mut(
            &mut image,
            Rgba([0u8, 0u8, 0u8, 0u8]),
            10,
            80,
            scale,
            &font,
            &data.to_string(),
        );
        draw_text_mut(
            &mut image,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            11,
            81,
            scale,
            &font,
            &data.to_string()
        );

        // save modified image
        match image.save(&file) {
            Ok(()) => Ok(()),
            Err(_e) => Err(PictureError::new(PictureErrorType::IO)),
        }
    }
}
