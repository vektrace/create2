#![doc = include_str!("../README.md")]

mod clean;
mod dock;
mod error;
mod mode;
mod opcode;
mod schedule;
mod serial;

pub use error::{Error, Result};
pub use mode::OperatingMode;
pub use opcode::Opcode;
pub use schedule::{Day, Schedule, ScheduleIter, Time, WeeklyTime};
pub use serial::BaudRate;
use std::io::{Read, Write};

/// iRobot Create 2 interface
///
/// Commands can be sent in any operating mode. A command may be ignored by the robot if the current
/// operating mode does not support it. See the respective command documentation for details on
/// which operating modes are supported by each command.
pub struct Create<T: Write + Read> {
    port: T,
}

impl<T: Write + Read> Create<T> {
    /// Wraps a port as a Create 2 interface
    pub fn new(port: T) -> Self {
        Self { port }
    }

    /// Returns a reference to the port commands are sent through
    pub fn port(&self) -> &T {
        &self.port
    }

    /// Returns a mutable reference to the port commands are sent through
    pub fn port_mut(&mut self) -> &mut T {
        &mut self.port
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        Ok(self.port.write_all(data).and_then(|_| self.port.flush())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn send() {
        let mut create = Create::new(Cursor::new(Vec::new()));

        create.send(&[1]).expect("Sending should not fail");

        assert_eq!(create.port().get_ref(), &[1]);
    }
}
