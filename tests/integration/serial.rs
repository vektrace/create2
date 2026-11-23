use crate::common::MockPort;
use create2::{BaudRate, Create, Opcode};

#[test]
fn set_baud() {
    let mut create = Create::new(MockPort::default());

    let baud = BaudRate::B19200;
    create.set_baud(baud).unwrap();

    assert_eq!(create.port().to_robot, &[Opcode::Baud as u8, baud as u8]);
}
