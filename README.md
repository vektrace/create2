# create2

A Rust library for controlling iRobot Create 2 robots.

## Getting started

You need a serial connection to the Create robot.

```rust
use create2::{Create, mode::SetOperatingMode};

// Since the port is only required to implement Write and Read, any other
// crate could be used too. The serialport crate is used here.
let port = serialport::new("/dev/ttyUSB0", 115200).open().expect("Failed to open port");

let mut create = Create::new(port);

create.start()?;

create.set_mode(SetOperatingMode::Safe)?;

// Do other operations...

create.stop()?;
```

There are more examples in the [examples] directory.
