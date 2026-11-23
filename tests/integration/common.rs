use std::io::{Error as IoError, ErrorKind, Read, Result, Write};

#[derive(Debug, Default)]
pub struct MockPort {
    /// Data sent to the robot
    pub to_robot: Vec<u8>,
    /// Data received from the robot
    pub from_robot: Vec<u8>,
    /// Port always fails with the set error
    pub fail: Option<ErrorKind>,
}

impl Write for MockPort {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(err) = self.fail {
            return Err(err.into());
        }

        self.to_robot.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        if let Some(err) = self.fail {
            return Err(err.into());
        }

        Ok(())
    }
}

impl Read for MockPort {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(err) = self.fail {
            return Err(err.into());
        }

        if self.from_robot.is_empty() {
            return Err(IoError::new(ErrorKind::WouldBlock, "no data available"));
        }

        let n = buf.len().min(self.from_robot.len());
        buf[..n].copy_from_slice(&self.from_robot[..n]);
        self.from_robot.drain(..n);
        Ok(n)
    }
}
