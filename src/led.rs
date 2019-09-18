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

// Status LED control

extern crate sysfs_gpio;
use std::thread;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

#[allow(dead_code)]
pub struct LED {
    pub pin: Pin,
}

impl LED {
    pub fn new(p: u8) -> LED {
        LED {
            pin: Pin::new(p as u64),
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // export the pin and set it as an output
        match self.pin.export() {
            Ok(()) => {}
            Err(_err) => return Err("Can't export pin"),
        }

        match self.pin.set_direction(Direction::Out) {
            Ok(()) => {}
            Err(_err) => return Err("Can't set gpio direction"),
        }

        // pull the pin low
        match self.pin.set_value(0) {
            Ok(()) => {}
            Err(_err) => return Err("Can't set pin value"),
        }

        Ok(())
    }

    // fast blink
    pub fn blink(&mut self) {
        self.pin.set_value(1).unwrap();
        thread::sleep(Duration::from_millis(1));
        self.pin.set_value(0).unwrap();
    }

    // error
    pub fn err(&mut self) {
        for _i in 0..5 {
            self.pin.set_value(1).unwrap();
            thread::sleep(Duration::from_millis(1));
            self.pin.set_value(0).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        // keep it on
        self.pin.set_value(1).unwrap();
    }
}
