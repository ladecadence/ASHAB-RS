extern crate chrono;

use chrono::*;
use std::io::prelude::*;
use std::fs::OpenOptions;


pub enum LogType {
	Data,
	Info,
	Warn,
	Err,
}

pub struct Log {
	pub filename: String,
}


impl Log {
	pub fn new (f: &str) -> Log {
		Log {
			filename: String::from(f),
		}

	}

	pub fn init(&mut self) {
		// create new file or erase if it exists
		let f = OpenOptions::new()
			.create(true)
			.truncate(true)
			.write(true)
			.open(self.filename.as_str()).unwrap();
	}

	pub fn log (&mut self, t: LogType, msg: &str) {
		// open file for append
		let mut f = OpenOptions::new()
			.append(true)
			.create(true)
			.open(self.filename.as_str()).unwrap();
		// log msg
		match t {
			LogType::Data => f.write_all(b"DATA::").unwrap(),
			LogType::Info => f.write_all(b"INFO::").unwrap(),
			LogType::Warn => f.write_all(b"WARN::").unwrap(),
			LogType::Err  => f.write_all(b" ERR::").unwrap(),
		}
		f.write_all(UTC::now().to_rfc3339().as_bytes()).unwrap();
		f.write_all(b":: ").unwrap();
		f.write_all(msg.as_bytes()).unwrap();
		f.write_all(b"\n").unwrap();
	}
}



