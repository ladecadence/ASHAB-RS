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

// Creates and updates a log file of the mission status or data

extern crate chrono;

use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub enum LogType {
    Data,
    Info,
    Warn,
    Error,
    Clean, // No prefix or timestamp
}

#[allow(dead_code)]
pub struct Log {
    pub filename: String,
}

impl Log {
    pub fn new() -> Self {
        Self {
            filename: String::from(""),
        }
    }

    pub fn init(&mut self, f: &str) {
        // add timestamp to filename
        self.filename = f.to_string() + &Utc::now().to_rfc3339() + ".log";
        // create new file or erase if it exists
        let _f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.filename.as_str())
            .unwrap();
    }

    pub fn log(&mut self, t: LogType, msg: &str) -> Result<(), io::Error>{
        // open file for append
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.filename.as_str())?;
        // log msg
        // headers
        match t {
            LogType::Data => f.write_all(b"DATA::")?,
            LogType::Info => f.write_all(b"INFO::")?,
            LogType::Warn => f.write_all(b"WARN::")?,
            LogType::Error => f.write_all(b" ERR::")?,
            LogType::Clean => {},
        }
        // and timestamp
        match t {
            LogType::Clean => {},
            _ => {
                f.write_all(Utc::now().to_rfc3339().as_bytes())?;
                f.write_all(b":: ")?;
            },
        }
        // write message
        f.write_all(msg.as_bytes())?;
        f.write_all(b"\n")?;
        // try to sync all data so it isn't lost in a power down/reset event
        f.sync_all()?;
        Ok(())
    }
}
