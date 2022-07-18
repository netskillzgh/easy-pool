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

    #[inline]
    pub fn create(self: &Arc<Self>) -> PoolObjectContainer<T> {
        self.create_with(|| Default::default())
    }

    /// The function that you pass as an argument does not affect the object if it is already present in memory.
    /// It is interesting for example when creating a "vector" to specify a capacity.
    pub fn create_with<F: FnOnce() -> T>(self: &Arc<Self>, f: F) -> PoolObjectContainer<T> {
        let val = self.values.lock().pop().unwrap_or_else(f);
        PoolObjectContainer::new(val, PoolType::Mutex(Arc::clone(&self)))
    }

    #[inline]
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

    #[test]
    fn test_create_with() {
        let pool = Arc::new(PoolMutex::<Vec<u8>>::new());
        let r = pool.create_with(|| Vec::with_capacity(4096));
        assert_eq!(r.capacity(), 4096);
    }

    #[test]
    fn test_create() {
        let pool = Arc::new(PoolMutex::<Vec<u8>>::new());
        let r = pool.create();
        assert_eq!(r.capacity(), Vec::<u8>::default().capacity());
    }

    #[test]
    fn test_len() {
        let pool = Arc::new(PoolMutex::<Vec<u8>>::new());
        assert_eq!(pool.len(), 0);
        let new_vec = pool.create();
        drop(new_vec);
        assert_eq!(pool.len(), 1);
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
        self.clear();
        debug_assert!(self.is_empty());
    }
}
