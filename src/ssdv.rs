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

// Uses the SSDV binary to encode pictures to be sent over a radio link

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::process::{Command, Stdio};

// ssdv program
// https://github.com/fsphil/ssdv
static SSDV_PROGRAM: &'static str = "ssdv";

// Errors
#[derive(Debug)]
pub enum SSDVErrorType {
    External,
    IO,
    Access,
}

#[derive(Debug)]
pub struct SSDVError {
    error_type: SSDVErrorType,
}

impl SSDVError {
    pub fn new(e: SSDVErrorType) -> Self {
        Self { error_type: e }
    }
}

pub struct SSDV {
    pub image_file: String,
    pub id: String,
    pub count: u8,
    filename: String,
    pub binaryname: String,
    pub packets: u64,
}

impl SSDV {
    pub fn new(img: String, p: String, b: String, i: String, cnt: u8) -> Self {
        Self {
            image_file: img.clone(),
            id: i,
            count: cnt,
            filename: img,
            binaryname: p + &b + ".bin",
            packets: 0,
        }
    }

    pub fn encode(&mut self) -> Result<(), SSDVError> {
        let status = Command::new(SSDV_PROGRAM)
            .arg("-e")
            .arg("-c")
            .arg(&self.id)
            .arg("-i")
            .arg(&format!("{}", self.count))
            .arg(&self.filename)
            .arg(&self.binaryname)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let exit_code: i32;
        match status {
            Ok(s) => exit_code = s.code().unwrap(),
            Err(_e) => return Err(SSDVError::new(SSDVErrorType::IO)),
        }

        // ssdv worked, get number of packets and return
        if exit_code == 0 {
            let _f = match fs::metadata(&self.binaryname) {
                Ok(f) => {
                    // get number of packets of the file
                    self.packets = f.len() / 256;
                    return Ok(());
                }
                Err(_e) => return Err(SSDVError::new(SSDVErrorType::IO)),
            };
        } else {
            // exit code not 0
            Err(SSDVError::new(SSDVErrorType::External))
        }
    }

    pub fn get_packet(&mut self, packet: u64) -> Result<[u8; 255], SSDVError> {
        // return if no packets
        if self.packets == 0 {
            return Err(SSDVError::new(SSDVErrorType::Access));
        }

        // return if invalid index
        if packet > self.packets {
            return Err(SSDVError::new(SSDVErrorType::Access));
        }

        // Ok, get packet
        let mut _file = match File::open(&self.binaryname) {
            Ok(mut f) => {
                // move the cursor, we can unwrap as we already checked number of packets
                // don't read first byte from packet (sync byte)
                f.seek(SeekFrom::Start((packet * 256) + 1)).unwrap();
                // read the buffer
                let mut buf = [0; 255];
                f.read(&mut buf).unwrap();
                return Ok(buf);
            }
            Err(_e) => return Err(SSDVError::new(SSDVErrorType::IO)),
        };
    }
}
