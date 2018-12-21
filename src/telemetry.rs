extern crate chrono;
use chrono::prelude::*;

pub struct Telemetry {
	id: String,
	msg: String,
	lat: f32,
	ns: char,
	lon: f32,
	ew: char,
	alt: f32, 
	hdg: f32,
	spd: f32,
	sats: u8,
	vbat: f32,
	baro: f32,
	tin: f32,
	tout: f32,
	arate: f32,
	date: String,
	time: String,
	sep: String,
    date_time: DateTime<Utc>,

}

impl Telemetry {
	pub fn new (i: String, m: String,
			s: String) -> Telemetry {

		Telemetry {
			id: i,
			msg: m,
			sep: s,
			lat: 0.0,
			ns: 'N',
			lon: 0.0,
			ew: 'W',
			alt: 0.0,
			hdg: 0.0,
			spd: 0.0,	
			sats: 0,
			vbat: 0.0,
			baro: 0.0,
			tin: 0.0,
			tout: 0.0,
			arate: 0.0,
            date_time: Utc::now(),
			date: format!("{:02}-{:02}-{}",
                Utc::now().day(), Utc::now().month(),
                Utc::now().year()),
			time: format!("{:02}:{:02}:{02}",
                Utc::now().hour(), Utc::now().minute(),
                Utc::now().second()),
		}
	}

	pub fn update(&mut self, lat: f32, ns: char,
			lon: f32, ew: char, alt: f32,
			hdg: f32, spd: f32, sats: u8,
			vbat: f32, baro: f32, tin: f32, 
			tout: f32) {

        // save old altitude for ascension rate
        let old_alt = self.alt;

		// update fields
		self.lat = lat;
		self.ns = ns;
		self.lon = lon;
		self.ew = ew;
		self.alt = alt;
		self.hdg = hdg;
		self.spd = spd;
		self.sats = sats;
		self.vbat = vbat;
		self.baro = baro;
		self.tin = tin;
		self.tout = tout;
	
        // save old datetime
        let old_date_time = self.date_time;

		// update packet date
		self.date_time = Utc::now();
		self.date = format!("{:02}-{:02}-{}",
					self.date_time.day(),
					self.date_time.month(),
					self.date_time.year()
					);
		self.time = format!("{:02}:{:02}:{:02}",
					self.date_time.hour(),
					self.date_time.minute(),
					self.date_time.second()
					);

        // calculate ascension rate
        let delta_time = self.date_time.signed_duration_since(old_date_time);
        if delta_time.num_milliseconds() != 0 {
            self.arate = (self.alt - old_alt) as f32 / 
                            (delta_time.num_milliseconds() as f32 / 1000.0);
        } else {
            self.arate = 0.0;
        }


	}

	fn dec_lat(&self) -> f32 {
		let degrees = (self.lat/100.0).trunc();
		let fraction = (self.lat - (degrees*100.0)) / 60.0;
	        degrees + fraction
	}

	fn dec_lon(&self) -> f32 {
	        let degrees = (self.lon/100.0).trunc();
	        let fraction = (self.lon - (degrees*100.0)) / 60.0;
	        degrees + fraction
	}

	pub fn aprs_string(&mut self) -> String {
		// gen APRS coordinates
		let coords = format!("{:07.2}{}{}{:08.2}{}", 
					self.lat,
					self.ns,
					self.sep,
					self.lon,
					self.ew);
		
		let mut aprs = String::from("$$");
		aprs.push_str(&self.id);
		aprs.push_str("!");
		aprs.push_str(&coords);
		aprs.push_str("O");
		aprs.push_str(&format!("{:.1}", self.hdg));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("{:.1}", self.spd));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("A={:.1}", self.alt));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("V={:.2}", self.vbat));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("P={:.1}", self.baro));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("TI={:.1}", self.tin));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("TO={:.1}", self.tout));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("{}", self.date));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("{}", self.time));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("GPS={:09.6}{},{:010.6}{}",
					self.dec_lat(),
					self.ns,
					self.dec_lon(),
					self.ew
					));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("SATS={}", self.sats));
		aprs.push_str(&self.sep);
		aprs.push_str(&format!("AR={:.1}", self.arate));
		aprs.push_str(&self.sep);
		aprs.push_str(&self.msg.replace("\n", " - "));
		aprs.push_str("\n");

		// fill with nulls up to 255 chars
		if aprs.len() < 255 {
			while aprs.len() < 255 {
				aprs.push_str("\0");
			}
		}
	
		// return string
		aprs

	}
}
