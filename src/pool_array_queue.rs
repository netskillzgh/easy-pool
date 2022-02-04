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

    pub fn create(self: &Arc<Self>) -> PoolObjectContainer<T> {
        let val = self.values.pop().unwrap_or_default();
        PoolObjectContainer::new(val, PoolType::ArrayQueue(Arc::clone(&self)))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T: Default + Clear> Default for PoolArrayQueue<T> {
    fn default() -> Self {
        Self::new(4096)
    }
}
