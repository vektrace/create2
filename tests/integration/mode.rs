use crate::common::MockPort;
use create2::{Create, Opcode, OperatingMode};

#[test]
fn start() {
    let mut create = Create::new(MockPort::default());

    create.start().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Start as u8]);
}

#[test]
fn reset() {
    let mut create = Create::new(MockPort::default());

    create.reset().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Reset as u8]);
}

#[test]
fn stop() {
    let mut create = Create::new(MockPort::default());

    create.stop().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Stop as u8]);
}

#[test]
fn set_mode() {
    let mut create = Create::new(MockPort::default());

    create.set_mode(OperatingMode::Off).unwrap();
    create.set_mode(OperatingMode::Passive).unwrap();
    create.set_mode(OperatingMode::Safe).unwrap();
    create.set_mode(OperatingMode::Full).unwrap();

    assert_eq!(
        create.port().to_robot,
        &[
            Opcode::Stop as u8,
            Opcode::Start as u8,
            Opcode::Safe as u8,
            Opcode::Full as u8
        ]
    );
}

#[test]
fn power_down() {
    let mut create = Create::new(MockPort::default());

    create.power_down().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Power as u8]);
}
