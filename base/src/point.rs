mod chunk;
mod cluster;
mod global;

pub use crate::point::{
    chunk::Point as ChunkPoint, cluster::Point as ClusterPoint, global::Point as GlobalPoint,
};
use std::{error, fmt, num::TryFromIntError};

#[derive(Debug, Eq, PartialEq)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong coordinates")
    }
}

impl error::Error for Error {}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self
    }
}
