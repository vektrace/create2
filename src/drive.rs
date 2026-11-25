use crate::{Create, Error, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Controls the robot's drive wheels.
    ///
    /// See [`Radius`] for more details.
    ///
    /// Note that internal and environmental restrictions may prevent the robot from accurately
    /// executing some drive commands.
    ///
    /// Available in modes: [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// Velocity must be in the range of -500 - 500 mm/s.
    /// Radius must be in the range of -2000 - 2000 mm.
    pub fn drive(&mut self, velocity: i16, radius: Radius) -> Result<()> {
        if !(-500..=500).contains(&velocity) {
            return Err(Error::InvalidVelocity);
        }

        let radius = radius.to_bits()?;

        let [velocity_high, velocity_low] = velocity.to_be_bytes();
        let [radius_high, radius_low] = radius.to_be_bytes();

        self.send(&[
            Opcode::Drive as u8,
            velocity_high,
            velocity_low,
            radius_high,
            radius_low,
        ])
    }

    pub fn drive_direct(&mut self, left_velocity: i16, right_velocity: i16) -> Result<()> {
        if !(-500..=500).contains(&left_velocity) || !(-500..=500).contains(&right_velocity) {
            return Err(Error::InvalidVelocity);
        }

        let [right_high, right_low] = right_velocity.to_be_bytes();
        let [left_high, left_low] = left_velocity.to_be_bytes();

        self.send(&[
            Opcode::DriveDirect as u8,
            right_high,
            right_low,
            left_high,
            left_low,
        ])
    }
}

/// Turn radius for [`Create::drive`]
pub enum Radius {
    /// Radius in mm measured from the center of the turning circle to the robot's center. Positive
    /// turns left, negative turns right.
    ///
    /// Must be in the range of -2000 - 2000 mm.
    Turn(i16),
    /// Drive straight.
    Straight,
    /// Rotate clockwise in place.
    SpinClockwise,
    /// Rotate counter-clockwise in place.
    SpinCounterClockwise,
}

impl Radius {
    fn to_bits(&self) -> Result<u16> {
        match self {
            Radius::Turn(r) if (-2000..=2000).contains(r) => Ok(*r as u16),
            Radius::Turn(_) => Err(Error::InvalidRadius),
            Radius::Straight => Ok(0x7FFF),
            Radius::SpinClockwise => Ok(0xFFFF),
            Radius::SpinCounterClockwise => Ok(0x0001),
        }
    }
}
