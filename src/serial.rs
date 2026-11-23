use crate::{Create, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Sets the baud rate in bits per second (bps) at which commands and data are sent.
    ///
    /// The default rate at power up is 115200. It can be changed to 19200 by following the method
    /// on page 4 in the [Create 2 Open Interface Spec](https://cdn-shop.adafruit.com/datasheets/create_2_Open_Interface_Spec.pdf).
    ///
    /// Once the baud rate is changed, it persists until the robot is power cycled by pressing the
    /// power button or removing the battery. You must wait 100ms after sending this command before
    /// sending other commands at the new rate.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive), [`Safe`](crate::OperatingMode::Safe),
    /// [`Full`](crate::OperatingMode::Full)
    pub fn set_baud(&mut self, baud: BaudRate) -> Result<()> {
        self.send(&[Opcode::Baud as u8, baud as u8])
    }
}

/// OI baud rates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(missing_docs, reason = "variant names are baud rate names")]
#[repr(u8)]
pub enum BaudRate {
    B300 = 0,
    B600 = 1,
    B1200 = 2,
    B2400 = 3,
    B4800 = 4,
    B9600 = 5,
    B14400 = 6,
    B19200 = 7,
    B28800 = 8,
    B38400 = 9,
    B57600 = 10,
    /// Default
    B115200 = 11,
}
