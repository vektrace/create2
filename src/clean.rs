use crate::{Create, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Starts the default cleaning mode.
    ///
    /// This is the same as pressing the robot's Clean button.
    /// It will pause a cleaning cycle if one is already in progress.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](crate::OperatingMode::Passive)
    pub fn clean(&mut self) -> Result<()> {
        self.send(&[Opcode::Clean as u8])
    }

    /// Starts the Max cleaning mode (cleans until the battery is dead).
    ///
    /// It will pause a cleaning cycle if one is already in progress.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](crate::OperatingMode::Passive)
    pub fn max_clean(&mut self) -> Result<()> {
        self.send(&[Opcode::Max as u8])
    }

    /// Starts the Spot cleaning mode.
    ///
    /// This is the same as pressing the robot's Spot button.
    /// It will pause a cleaning cycle if one is already in progress.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](crate::OperatingMode::Passive)
    pub fn spot_clean(&mut self) -> Result<()> {
        self.send(&[Opcode::Spot as u8])
    }

    /// Directs robot to drive onto the dock the next time it encounters the docking beams.
    ///
    /// This is the same as pressing the robot's Dock button.
    /// It will pause a cleaning cycle if one is already in progress.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Changes mode to [`Passive`](crate::OperatingMode::Passive)
    pub fn seek_dock(&mut self) -> Result<()> {
        self.send(&[Opcode::SeekDock as u8])
    }
}
