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
