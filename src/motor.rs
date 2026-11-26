use crate::{Create, Opcode, Result};
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
