use crate::common::MockPort;
use create2::{Create, Error, Opcode, Radius};

#[test]
fn drive() {
    let mut create = Create::new(MockPort::default());

    create.drive(500, Radius::Turn(10)).unwrap();
    assert_eq!(
        create.port().to_robot,
        &[Opcode::Drive as u8, 1, 244, 0, 10]
    );
    create.port_mut().to_robot.clear();

    create.drive(-500, Radius::Straight).unwrap();
    assert_eq!(
        create.port().to_robot,
        &[Opcode::Drive as u8, 254, 12, 127, 255]
    );
    create.port_mut().to_robot.clear();

    create.drive(200, Radius::SpinClockwise).unwrap();
    assert_eq!(
        create.port().to_robot,
        &[Opcode::Drive as u8, 0, 200, 255, 255]
    );
    create.port_mut().to_robot.clear();

    create.drive(-200, Radius::SpinCounterClockwise).unwrap();
    assert_eq!(
        create.port().to_robot,
        &[Opcode::Drive as u8, 255, 56, 0, 1]
    );

    assert!(matches!(
        create.drive(600, Radius::Straight),
        Err(Error::InvalidVelocity)
    ));
    assert!(matches!(
        create.drive(500, Radius::Turn(4000)),
        Err(Error::InvalidRadius)
    ));
}
