use std::{fmt::Debug, sync::Arc};

use crossbeam::queue::ArrayQueue;

use crate::{Clear, PoolObjectContainer, PoolType};

#[derive(Debug)]
pub struct PoolArrayQueue<T> {
    pub(crate) values: ArrayQueue<T>,
    pub(crate) max_size: usize,
}

impl<T> PoolArrayQueue<T>
where
    T: Default + Clear,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            values: ArrayQueue::new(capacity),
            max_size: capacity,
        }
    }

    #[inline]
    pub fn create(self: &Arc<Self>) -> PoolObjectContainer<T> {
        self.create_with(|| Default::default())
    }

    pub fn create_with<F: FnOnce() -> T>(self: &Arc<Self>, f: F) -> PoolObjectContainer<T> {
        let val = self.values.pop().unwrap_or_else(f);
        PoolObjectContainer::new(val, PoolType::ArrayQueue(Arc::clone(&self)))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T: Default + Clear> Default for PoolArrayQueue<T> {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pool = Arc::new(PoolArrayQueue::<Vec<u8>>::new(1024));
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
