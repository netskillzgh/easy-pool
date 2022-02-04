use std::{
    fmt::Debug,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::Arc,
};

use crossbeam::queue::ArrayQueue;

use crate::Clear;

#[derive(Debug)]
pub struct PoolArrayQueue<T> {
    values: ArrayQueue<T>,
    max_size: usize,
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
        PoolObjectContainer::new(val, Arc::clone(&self))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug)]
pub struct PoolObjectContainer<T: Clear> {
    inner: ManuallyDrop<T>,
    ref_pool: Arc<PoolArrayQueue<T>>,
}

impl<T: Clear> PoolObjectContainer<T> {
    fn new(val: T, ref_pool: Arc<PoolArrayQueue<T>>) -> Self {
        Self {
            inner: ManuallyDrop::new(val),
            ref_pool,
        }
    }
}

impl<T: Clear + Default> DerefMut for PoolObjectContainer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clear + Default> Deref for PoolObjectContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clear> Drop for PoolObjectContainer<T> {
    fn drop(&mut self) {
        let val = unsafe { ptr::read(&self.inner as *const ManuallyDrop<T>) };
        let mut val = ManuallyDrop::into_inner(val);

        let lock = &self.ref_pool.values;

        if lock.len() >= self.ref_pool.max_size {
            drop(val);
        } else {
            val.clear();
            if let Err(val) = lock.push(val) {
                drop(val);
            }
        }
    }
}

impl<T: Default + Clear> Default for PoolArrayQueue<T> {
    fn default() -> Self {
        Self::new(4096)
    }
}
