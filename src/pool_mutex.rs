use std::sync::Arc;

use parking_lot::Mutex;

use crate::{Clear, PoolObjectContainer, PoolType};

#[derive(Debug)]
pub struct PoolMutex<T> {
    pub(crate) values: Mutex<Vec<T>>,
    pub(crate) max_size: usize,
}

impl<T> PoolMutex<T>
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
        PoolObjectContainer::new(val, PoolType::Mutex(Arc::clone(&self)))
    }

    pub fn len(&self) -> usize {
        self.values.lock().len()
    }
}

impl<T: Default + Clear> Default for PoolMutex<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pool = Arc::new(PoolMutex::<Vec<u8>>::new());
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
