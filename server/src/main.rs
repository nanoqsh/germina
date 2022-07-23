mod kit;

use crate::kit::Kit;

fn main() {
    let kit = match Kit::load("a.kit".as_ref()) {
        Ok(kit) => kit,
        Err(_) => todo!(),
    };

    let tiles = kit.model.tiles.into_inner();
    for key in tiles.keys() {
        println!("{key:?}");
    }
}
