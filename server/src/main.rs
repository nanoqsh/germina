mod error;
mod load;

use crate::{error::Error, load::Kit};

fn main() {
    env_logger::init();

    let path = "base.kit".as_ref();
    let kit = match Kit::load(path).map_err(|err| Error::Load {
        err,
        path: path.into(),
    }) {
        Ok(kit) => kit,
        Err(err) => err.exit(),
    };

    assert!(kit.model.tiles.get("test").is_some());
}
