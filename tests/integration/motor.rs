use crate::common::MockPort;
use create2::{Create, MainBrushDirection, Opcode, SideBrushDirection};

#[test]
fn motors() {
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

#[test]
fn motors_pwm() {
    let mut create = Create::new(MockPort::default());

    create.motors_pwm(64, -127, 127).unwrap();

    assert_eq!(
        create.port().to_robot,
        &[Opcode::PWMMotors as u8, 64, -127i8 as u8, 127]
    )
}
