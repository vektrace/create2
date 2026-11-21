use crate::{Create, Day, Opcode, Result, Schedule, Time};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Sends the robot a new schedule.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Note that if the robot's schedule or clock button is pressed, the command will be ignored.
    pub fn schedule(&mut self, schedule: Schedule) -> Result<()> {
        let mut buf = [0u8; 16];
        buf[0] = Opcode::Schedule as u8;

        for wt in &schedule {
            let index = wt.day as usize;
            buf[1] |= wt.day.bit();
            buf[2 + index * 2] = wt.time.hour();
            buf[2 + index * 2 + 1] = wt.time.minute();
        }

        self.send(&buf)
    }

    /// Sets the robots clock.
    ///
    /// Available in modes: [`Passive`](crate::OperatingMode::Passive),
    /// [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Note that if the robot's schedule or clock button is pressed, the command will be ignored.
    pub fn set_daytime(&mut self, day: Day, time: Time) -> Result<()> {
        self.send(&[
            Opcode::SetDayTime as u8,
            day as u8,
            time.hour(),
            time.minute(),
        ])
    }
}
