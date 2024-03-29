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

// Object that stores the data in the telemetry packets and generates
// formatted strings to store/send them.

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
    hpwr: u8,
}

impl Telemetry {
    pub fn new(i: String, m: String, s: String) -> Self {
        Self {
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
            date: format!(
                "{:02}-{:02}-{}",
                Utc::now().day(),
                Utc::now().month(),
                Utc::now().year()
            ),
            time: format!(
                "{:02}:{:02}:{02}",
                Utc::now().hour(),
                Utc::now().minute(),
                Utc::now().second()
            ),
            hpwr: 0,
        }
    }

    pub fn update(
        &mut self,
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
        hpwr: u8,
    ) {
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
        self.hpwr = hpwr;

        // save old datetime
        let old_date_time = self.date_time;

        // update packet date
        self.date_time = Utc::now();
        self.date = format!(
            "{:02}-{:02}-{}",
            self.date_time.day(),
            self.date_time.month(),
            self.date_time.year()
        );
        self.time = format!(
            "{:02}:{:02}:{:02}",
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
        let degrees = (self.lat / 100.0).trunc();
        let fraction = (self.lat - (degrees * 100.0)) / 60.0;
        degrees + fraction
    }

    fn dec_lon(&self) -> f32 {
        let degrees = (self.lon / 100.0).trunc();
        let fraction = (self.lon - (degrees * 100.0)) / 60.0;
        degrees + fraction
    }

    pub fn aprs_string(&mut self) -> String {
        // gen APRS coordinates
        let coords = format!(
            "{:07.2}{}{}{:08.2}{}",
            self.lat, self.ns, self.sep, self.lon, self.ew
        );

        let mut aprs = String::from("$$");
        aprs.push_str(&self.id);
        aprs.push('!');
        aprs.push_str(&coords);
        aprs.push('O');
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
        aprs.push_str(&format!(
            "GPS={:09.6}{},{:010.6}{}",
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
        aprs.push_str(&format!(" - {}", match self.hpwr {
            0 => "L",
            1 => "H",
            _ => "?",
        }));
        aprs.push_str("\n");

        // fill with nulls up to 255 chars
        //if aprs.len() < 255 {
        //    while aprs.len() < 255 {
        //        aprs.push_str("\0");
        //    }
        //}

        // return string
        aprs
    }

    pub fn csv_string(&mut self) -> String {
        let mut csv = String::from("");
        csv.push_str(&format!("{},", self.date));
        csv.push_str(&format!("{},", self.time));
        csv.push_str(&format!("{},", self.dec_lat()));
        csv.push_str(&format!("{},", self.ns));
        csv.push_str(&format!("{},", self.dec_lon()));
        csv.push_str(&format!("{},", self.ew));
        csv.push_str(&format!("{:.1},", self.alt));
        csv.push_str(&format!("{:.2},", self.vbat));
        csv.push_str(&format!("{:.1},", self.tin));
        csv.push_str(&format!("{:.1},", self.tout));
        csv.push_str(&format!("{:.1},", self.baro));
        csv.push_str(&format!("{:.1},", self.hdg));
        csv.push_str(&format!("{:.1},", self.spd));
        csv.push_str(&format!("{},", self.sats));
        csv.push_str(&format!("{:.1}", self.arate));

        // return string
        csv
    }
}
