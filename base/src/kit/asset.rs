use crate::kit::Key;

pub struct Asset {
    pub name: Key,
    pub kind: Kind,
}

impl Asset {
    pub fn parse_path(str: &str) -> Option<Self> {
        let (kind, filename) = str.rsplit_once('/')?;
        let (name, ext) = filename.split_once('.')?;

        if ext != "json" {
            return None;
        }

        let kind = match kind {
            "tiles" => Kind::Tile,
            _ => return None,
        };

        Some(Self {
            name: name.parse().ok()?,
            kind,
        })
    }
}

pub enum Kind {
    Tile,
}
