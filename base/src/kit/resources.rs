use fxhash::FxHashMap as Map;
use serde::Deserialize;
use std::{borrow, fmt, ops, rc::Rc, str};

#[derive(Clone, Deserialize, Hash, PartialEq, Eq)]
#[serde(try_from = "String")]
pub struct Key {
    inner: Rc<str>,
}

impl Key {
    fn from_str<S>(src: S) -> Result<Self, ParseError>
    where
        S: Into<String>,
    {
        fn is_valid(src: &str) -> bool {
            !src.is_empty()
                && src
                    .chars()
                    .all(|c| matches!(c, '0'..='9' | 'a'..='z' | 'A' ..='Z' | '_'))
        }

        let src = src.into();
        if is_valid(&src) {
            Ok(Self { inner: src.into() })
        } else {
            Err(ParseError(src))
        }
    }

    pub fn get(&self) -> &str {
        self
    }
}

impl TryFrom<String> for Key {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl str::FromStr for Key {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str(src)
    }
}

impl ops::Deref for Key {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl borrow::Borrow<str> for Key {
    fn borrow(&self) -> &str {
        self
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Resources<A> {
    map: Map<Key, A>,
}

impl<A> Resources<A> {
    pub fn insert(&mut self, key: Key, value: A) {
        if self.map.insert(key, value).is_some() {
            panic!("already has a key");
        }
    }

    pub fn try_insert<F, E>(&mut self, key: Key, callback: F) -> Result<(), E>
    where
        F: FnOnce(&Key) -> Result<A, E>,
    {
        use std::collections::hash_map::Entry;

        if let Entry::Vacant(en) = self.map.entry(key) {
            let value = callback(en.key())?;
            en.insert(value);
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&A> {
        self.map.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &A)> {
        self.map.iter()
    }
}

impl<A> Default for Resources<A> {
    fn default() -> Self {
        Self {
            map: Map::default(),
        }
    }
}
