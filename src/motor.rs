use crate::{Create, Error, Opcode, Result};
use std::io::{Read, Write};

impl<T: Write + Read> Create<T> {
    /// Controls the forward and backward motion of the robot's vacuum, main and side brush
    /// independently.
    ///
    /// Motors will run at maximum speed when enabled.
    ///
    /// Available in modes: [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    pub fn motors(
        &mut self,
        side_brush: bool,
        vacuum: bool,
        main_brush: bool,
        main_brush_direction: MainBrushDirection,
        side_brush_direction: SideBrushDirection,
    ) -> Result<()> {
        let motors = ((main_brush_direction as u8) << 4)
            | ((side_brush_direction as u8) << 3)
            | ((main_brush as u8) << 2)
            | ((vacuum as u8) << 1)
            | (side_brush as u8);

        self.send(&[Opcode::Motors as u8, motors])
    }

    /// Controls the speed of the robot's vacuum, main and side brush independently.
    ///
    /// Duty cycles are linear: `127` is 100% of battery voltage, `64` is ~50%, etc.
    ///
    /// The main and side brush can run in either direction while the vacuum only runs forward.
    ///
    /// Positive speeds turn the motor in its default (cleaning) direction. Default direction for
    /// the side brush is counterclockwise, for the main brush it is inward.
    ///
    /// Available in modes: [`Safe`](crate::OperatingMode::Safe), [`Full`](crate::OperatingMode::Full)
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidPWM`](Error::InvalidPWM) error if the main or side brush duty cycle is
    /// not in the range or -127 - 127 or if the vacuum duty cycle is not in the range of 0 - 127.
    pub fn motors_pwm(&mut self, main_brush: i8, side_brush: i8, vacuum: u8) -> Result<()> {
        if main_brush == i8::MIN || side_brush == i8::MIN || vacuum > 127 {
            return Err(Error::InvalidPWM);
        }

        self.send(&[
            Opcode::PWMMotors as u8,
            main_brush as u8,
            side_brush as u8,
            vacuum,
        ])
    }
}

/// The direction the robot's main brush spins in
#[repr(u8)]
pub enum MainBrushDirection {
    Inward = 0,
    Outward = 1,
}

/// The direction the robot's side brush spins in
#[repr(u8)]
pub enum SideBrushDirection {
    Counterclockwise = 0,
    Clockwise = 1,
}
