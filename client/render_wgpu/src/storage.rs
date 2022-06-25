use fxhash::FxHashMap as HashMap;

pub struct Storage<T> {
    map: HashMap<u32, T>,
    counter: u32,
}

impl<T> Storage<T> {
    pub fn insert(&mut self, value: T) -> u32 {
        let index = self.counter;
        self.counter = self.counter.wrapping_add(1);
        self.map.insert(index, value);
        index
    }

    pub fn get(&self, index: u32) -> &T {
        self.map.get(&index).expect("undefined index")
    }

    pub fn remove(&mut self, index: u32) {
        self.map.remove(&index);
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            counter: 0,
        }
    }
}
