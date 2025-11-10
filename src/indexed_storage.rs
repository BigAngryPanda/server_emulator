use std::ops::{
    Index,
    IndexMut
};

pub struct IndexedStorage<T> {
    data: Vec<T>,
    free_pos: Vec<usize>
}

impl<T> IndexedStorage<T> {
    pub fn new() -> IndexedStorage<T> {
        IndexedStorage {
            data: Vec::new(),
            free_pos: Vec::new()
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        match self.free_pos.pop() {
            Some(i) => {
                self.data[i] = value;
                i
            },
            None => {
                self.data.push(value);
                self.data.len() - 1
            }
        }
    }

    pub fn remove(&mut self, id: usize) {
       self.free_pos.push(id);
    }

    pub fn contains(&self, id: usize) -> bool {
        self.data.len() > id && !self.free_pos.contains(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty() || self.data.len() == self.free_pos.len()
    }
}

impl<T> Index<usize> for IndexedStorage<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for IndexedStorage<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
