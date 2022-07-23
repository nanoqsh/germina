mod kit;

use crate::kit::Kit;

fn main() {
    let kit = match Kit::load("base.kit".as_ref()) {
        Ok(kit) => kit,
        Err(_) => todo!(),
    };

    assert!(kit.model.tiles.get("test").is_some());
}
