# create2

A Rust library for controlling iRobot Create 2 robots.

## Getting started

You need a serial connection to the Create robot.

```rust,no_run
use create2::{Create, OperatingMode};

fn main() -> anyhow::Result<()> {
    // Since the port is only required to implement Write and Read, any other
    // crate could be used too. The serialport crate is used here.
    let port = serialport::new("/dev/ttyUSB0", 115200).open().expect("Failed to open port");

    let mut create = Create::new(port);

    create.start()?;

    create.set_mode(OperatingMode::Safe)?;

    // Do other operations...

    create.stop()?;

    Ok(())
}
```

There are more examples in the [examples](https://github.com/vektrace/create2/tree/main/examples) directory.
