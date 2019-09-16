extern crate chrono;

use chrono::prelude::*;
use std::fs::OpenOptions;
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
    pub fn new() -> Log {
        Log {
            filename: String::from(""),
        }
    }

    pub fn init(&mut self, f: &str) {
        self.filename = f.to_string() + &Utc::now().to_rfc3339() + ".log";
        // create new file or erase if it exists
        let _f = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.filename.as_str())
            .unwrap();
    }

    pub fn log(&mut self, t: LogType, msg: &str) {
        // open file for append
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.filename.as_str())
            .unwrap();
        // log msg
        match t {
            LogType::Data => f.write_all(b"DATA::").unwrap(),
            LogType::Info => f.write_all(b"INFO::").unwrap(),
            LogType::Warn => f.write_all(b"WARN::").unwrap(),
            LogType::Error => f.write_all(b" ERR::").unwrap(),
            LogType::Clean => {},
        }
        match t {
            LogType::Clean => {},
            _ => {
                f.write_all(Utc::now().to_rfc3339().as_bytes()).unwrap();
                f.write_all(b":: ").unwrap();
            },
        }
        f.write_all(msg.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
        f.sync_all().unwrap();
    }
}
