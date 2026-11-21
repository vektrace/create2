//! Rust library for controlling iRobot Create 2 robots
//!
//! # Getting started
//!
//! You need a serial connection to the Create robot.
//!
//! ```ignore
//! use create2::{Create, mode::OperatingMode};
//!
//! // Since the port is only required to implement Write and Read, any other
//! // crate could be used too. The serialport crate is used here.
//! let port = serialport::new("/dev/ttyUSB0", 115200).open().expect("Failed to open port");
//!
//! let mut create = Create::new(port);
//!
//! create.start()?;
//!
//! create.set_mode(OperatingMode::Safe)?;
//!
//! // Do other operations...
//!
//! create.stop()?;
//! ```
//!
//! There are more examples in the [examples](https://github.com/vektrace/create2/tree/main/examples) directory.

mod clean;
pub mod error;
pub mod mode;
pub mod types;

use error::*;
use std::io::{Read, Write};
pub use types::*;

/// iRobot Create 2 interface
pub struct Create<T: Write + Read> {
    port: T,
}

impl<T: Write + Read> Create<T> {
    pub fn new(port: T) -> Self {
        Self { port }
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        Ok(self.port.write_all(data).and_then(|_| self.port.flush())?)
    }
}
