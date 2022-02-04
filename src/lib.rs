use parking_lot::Mutex;
pub mod pool_array_queue;
pub mod pool_seg_queue;
use std::{
    fmt::Debug,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::Arc,
};

#[derive(Debug)]
pub struct Pool<T> {
    values: Mutex<Vec<T>>,
    max_size: usize,
}

impl<T> Pool<T>
where
    T: Default + Clear,
{
    pub fn new() -> Self {
        Self::with_config(0, 4096)
    }

    pub fn with_config(capacity: usize, max_size: usize) -> Self {
        Self {
            values: Mutex::new(Vec::with_capacity(capacity)),
            max_size,
        }
    }

    pub fn create(self: &Arc<Self>) -> PoolObjectContainer<T> {
        let val = self.values.lock().pop().unwrap_or_default();
        PoolObjectContainer::new(val, Arc::clone(&self))
    }

    pub fn len(&self) -> usize {
        self.values.lock().len()
    }
}

#[derive(Debug)]
pub struct PoolObjectContainer<T: Clear> {
    inner: ManuallyDrop<T>,
    ref_pool: Arc<Pool<T>>,
}

impl<T: Clear> PoolObjectContainer<T> {
    fn new(val: T, ref_pool: Arc<Pool<T>>) -> Self {
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

        let mut lock = self.ref_pool.values.lock();

        if lock.len() >= self.ref_pool.max_size {
            drop(val);
        } else {
            val.clear();
            lock.push(val);
        }
    }
}

impl<T: Default + Clear> Default for Pool<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Clear {
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pool = Arc::new(Pool::<Vec<u8>>::new());
        let mut new_vec = pool.create();
        new_vec.extend_from_slice(&[0, 0, 0, 0, 1, 1]);
        let capacity = new_vec.capacity();
        drop(new_vec);
        assert!(!pool.values.lock().is_empty());
        let new_vec = pool.create();
        assert!(new_vec.capacity() > 0 && new_vec.capacity() == capacity);
        assert!(new_vec.is_empty());
        assert!(pool.values.lock().is_empty());
        drop(new_vec);
    }
}

impl Clear for String {
    #[inline]
    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Clear for Vec<T> {
    #[inline]
    fn clear(&mut self) {
        self.clear()
    }
}
