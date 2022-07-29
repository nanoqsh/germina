mod asset;
mod resources;

pub use self::{
    asset::{Asset, Kind},
    resources::{Key, ParseError as ParseKeyError, Resources},
};
