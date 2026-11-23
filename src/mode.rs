use crate::{Create, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Starts the OI. Must always be sent before sending any other commands.
    ///
    /// Available in modes: [`Passive`](OperatingMode::Passive), [`Safe`](OperatingMode::Safe),
    /// [`Full`](OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](OperatingMode::Passive). Robot beeps once to acknowledge it is starting from
    /// [`Off`](OperatingMode::Off) mode.
    pub fn start(&mut self) -> Result<()> {
        self.send(&[Opcode::Start as u8])
    }

    /// Resets the robot, as if the battery was removed and reinserted.
    ///
    /// Available in all modes.
    ///
    /// Changes mode to [`Off`](OperatingMode::Off). You will have to call [`start()`](Self::start()) again.
    pub fn reset(&mut self) -> Result<()> {
        self.send(&[Opcode::Reset as u8])
    }

    /// Stops the OI. Streams will be stopped and the robot will stop responding to commands.
    ///
    /// Use this command when you are finished working with the robot.
    ///
    /// Available in modes: [`Passive`](OperatingMode::Passive), [`Safe`](OperatingMode::Safe),
    /// [`Full`](OperatingMode::Full)
    ///
    /// Changes mode to [`Off`](OperatingMode::Off). Robot plays a song to acknowledge it is exiting the
    /// OI.
    pub fn stop(&mut self) -> Result<()> {
        self.send(&[Opcode::Stop as u8])
    }

    /// Set the operating mode of the Create 2.
    ///
    /// If the mode parameter is [`Off`](OperatingMode::Off) or [`Passive`](OperatingMode::Passive),
    /// the [`Stop`](Self::stop()) or [`Start`](Self::start()) command will be sent.
    ///
    /// See [`OperatingMode`] for more information.
    pub fn set_mode(&mut self, mode: OperatingMode) -> Result<()> {
        let opcode = match mode {
            OperatingMode::Off => Opcode::Stop,
            OperatingMode::Passive => Opcode::Start,
            OperatingMode::Safe => Opcode::Safe,
            OperatingMode::Full => Opcode::Full,
        };
        self.send(&[opcode as u8])
    }

    /// Powers down the robot.
    ///
    /// Available in modes: [`Passive`](OperatingMode::Passive), [`Safe`](OperatingMode::Safe),
    /// [`Full`](OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](OperatingMode::Passive)
    pub fn power_down(&mut self) -> Result<()> {
        self.send(&[Opcode::Power as u8])
    }
}

/// The operating modes of the OI
///
/// The robot starts in [`Off`](OperatingMode::Off).
///
/// Every command documents the modes it is available in and what the mode gets changed to.
///
/// Use [`Create::set_mode`] to change mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingMode {
    /// After a battery change or when power is first turned on, the OI is in Off mode.
    Off,
    /// The OI enters Passive mode when the [Start command](crate::Create::start()) or any of the cleaning mode commands has
    /// been sent.
    ///
    /// When in Passive mode, sensor data can be requested and received, but actuators (motors,
    /// speaker, lights, low side drivers, digital outputs) cannot be controlled. To control the
    /// actuators the robot must be either in [Full](OperatingMode::Full) or in
    /// [Safe](OperatingMode::Safe) mode.
    Passive,
    /// Safe mode gives full control of the robot, with some safety-related exceptions:
    /// - Detection of a cliff while moving forward (or moving backward with a small turning radius, less than one robot radius).
    /// - Detection of a wheel drop (on any wheel).
    /// - Charger plugged in and powered.
    ///
    /// If any of these conditions occur while in Safe mode, the robot stops all motors and reverts
    /// to [Passive](OperatingMode::Passive) mode.
    ///
    /// Charging terminates when you enter Safe Mode, and the robot will not power save.
    Safe,
    /// Full mode provides full control over the robot.
    ///
    /// The safety features are disabled in Full mode.
    ///
    /// Charging terminates when you enter Full Mode, and Roomba will not power save.
    Full,
}
