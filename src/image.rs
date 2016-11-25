use std::io;
use std::process::Command;
use std::io::prelude::*;
use std::process::ExitStatus;

static STILL_PROGRAM: &'static str = "raspistill";

#[allow(dead_code)]
pub struct Image {
	pub number: u32,
	pub basename: String,
	pub path: String,
	pub filename: String,
}

#[allow(dead_code)]
impl Image {
	pub fn new(num: u32, name: &str, p: &str) -> Image {
		Image {
			number : num,
			filename : String::from(p) + "/" + name.clone() + &num.to_string() + ".jpg",
			basename : String::from(name),
			path : String::from(p),
		}
	}

	fn update_name(&mut self) {
		self.filename = self.path.clone() + "/" + &self.basename + &self.number.to_string() + ".jpg";
	}

	pub fn capture(&mut self) -> bool {
		
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
			Err(e) => { println!("{}", e); return false }
		}
		
		// if we manage to capture a picture, increment filename number		
		if exit_code == 0 {
			self.number = self.number + 1;
			return true;
		}
		
		// exit code was not 0
		false
		
	}
}
		
