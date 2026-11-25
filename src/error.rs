use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// The hour or minute parameter passed into [`Time::new`](crate::Time::new()) is invalid.
    #[error("invalid time")]
    InvalidTime,
    /// The velocity is not in the range of -500 - 500 mm/s.
    #[error("invalid velocity")]
    InvalidVelocity,
    /// The radius is not in the range of -2000 - 2000 mm.
    #[error("invalid radius")]
    InvalidRadius,
}
