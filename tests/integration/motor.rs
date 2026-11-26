use crate::common::MockPort;
use create2::{Create, MainBrushDirection, Opcode, OperatingMode, SideBrushDirection};

#[test]
fn start() {
    let mut create = Create::new(MockPort::default());

    create
        .motors(
            true,
            false,
            true,
            MainBrushDirection::Inward,
            SideBrushDirection::Clockwise,
        )
        .unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Motors as u8, 13]);
}
