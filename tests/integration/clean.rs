use crate::common::MockPort;
use create2::{Create, Opcode};

#[test]
fn clean() {
    let mut create = Create::new(MockPort::default());

    create.clean().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Clean as u8]);
}

#[test]
fn max_clean() {
    let mut create = Create::new(MockPort::default());

    create.max_clean().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Max as u8]);
}

#[test]
fn spot_clean() {
    let mut create = Create::new(MockPort::default());

    create.spot_clean().unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Spot as u8]);
}
