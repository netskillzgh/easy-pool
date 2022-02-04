use std::{fmt::Debug, sync::Arc};

use crossbeam::queue::SegQueue;

use crate::{Clear, PoolObjectContainer, PoolType};

#[derive(Debug)]
pub struct PoolSegQueue<T> {
    pub(crate) values: SegQueue<T>,
    pub(crate) max_size: usize,
}

impl<T> PoolSegQueue<T>
where
    T: Default + Clear,
{
    pub fn new() -> Self {
        Self::with_config(4096)
    }

    pub fn with_config(max_size: usize) -> Self {
        Self {
            values: SegQueue::new(),
            max_size,
        }
    }

    pub fn create(self: &Arc<Self>) -> PoolObjectContainer<T> {
        let val = self.values.pop().unwrap_or_default();
        PoolObjectContainer::new(val, PoolType::SegQueue(Arc::clone(&self)))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T: Default + Clear> Default for PoolSegQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pool = Arc::new(PoolSegQueue::<Vec<u8>>::new());
        let mut new_vec = pool.create();
        new_vec.extend_from_slice(&[0, 0, 0, 0, 1, 1]);
        let capacity = new_vec.capacity();
        drop(new_vec);
        assert!(!pool.values.is_empty());
        let new_vec = pool.create();
        assert!(new_vec.capacity() > 0 && new_vec.capacity() == capacity);
        assert!(new_vec.is_empty());
        assert!(pool.values.is_empty());
        drop(new_vec);
    }
}
