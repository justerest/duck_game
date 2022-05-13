use std::collections::LinkedList;

pub struct FollowBuffer {
    list: LinkedList<f32>,
    capacity: usize,
}

impl FollowBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            list: LinkedList::new(),
            capacity,
        }
    }

    pub fn push(&mut self, val: f32) {
        self.list.push_back(val);
        if self.list.len() > self.capacity {
            self.list.pop_front();
        }
    }

    pub fn mean(&self) -> f32 {
        self.list.iter().sum::<f32>() / self.list.len() as f32
    }

    pub fn last(&self) -> Option<f32> {
        self.list.back().cloned()
    }
}
