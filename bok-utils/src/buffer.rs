use std::iter::FromIterator;

pub struct Buffer<T: Clone> {
    vec: Vec<T>,
    cur: usize,
}

impl<T: Clone> Buffer<T> {
    pub fn new<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let vec: Vec<T> = Vec::from_iter(iter);
        let cur = 0;
        Self { vec, cur }
    }

    pub fn default() -> Self {
        Buffer {
            vec: Vec::new(),
            cur: 0,
        }
    }

    pub fn has_next(&mut self) -> bool {
        if self.vec.len() == self.cur {
            false
        } else {
            true
        }
    }

    pub fn next(&mut self) -> Option<T> {
        let item = self.vec.get(self.cur);
        self.cur += 1;
        if let Some(x) = item {
            Some(x.clone())
        } else {
            None
        }
    }
    pub fn prev(&mut self) -> Option<T> {
        self.cur -= 1;
        let item = self.vec.get(self.cur);
        if let Some(x) = item {
            Some(x.clone())
        } else {
            None
        }
    }
}