#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]

use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::Arc,
};

pub use easy_pool_proc_macro::{EasyPoolArayQueue, EasyPoolMutex, EasyPoolSegQueue};
pub use once_cell::sync::Lazy;
pub use pool_array_queue::PoolArrayQueue;
pub use pool_mutex::PoolMutex;
pub use pool_seg_queue::PoolSegQueue;

mod pool_array_queue;
mod pool_mutex;
mod pool_seg_queue;

pub trait Clear {
    fn clear(&mut self);
}

#[derive(Debug)]
enum PoolType<T: Clear> {
    Mutex(Arc<PoolMutex<T>>),
    SegQueue(Arc<PoolSegQueue<T>>),
    ArrayQueue(Arc<PoolArrayQueue<T>>),
}

#[derive(Debug)]
pub struct PoolObjectContainer<T: Clear> {
    ref_pool: PoolType<T>,
    inner: ManuallyDrop<T>,
}

impl<T: Clear> PoolObjectContainer<T> {
    fn new(val: T, ref_pool: PoolType<T>) -> Self {
        Self {
            inner: ManuallyDrop::new(val),
            ref_pool,
        }
    }
}

impl<T: Clear> DerefMut for PoolObjectContainer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clear> Deref for PoolObjectContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clear> Drop for PoolObjectContainer<T> {
    fn drop(&mut self) {
        let val = unsafe { ptr::read(&self.inner) };
        let mut val = ManuallyDrop::into_inner(val);

        match self.ref_pool {
            PoolType::Mutex(ref pool) => {
                let mut lock = pool.values.lock();

                if lock.len() >= pool.max_size {
                    drop(val);
                } else {
                    val.clear();
                    lock.push(val);
                }
                drop(lock);
            }
            PoolType::SegQueue(ref pool) => {
                if pool.values.len() >= pool.max_size {
                    drop(val);
                } else {
                    val.clear();
                    pool.values.push(val);
                }
            }
            PoolType::ArrayQueue(ref pool) => {
                if pool.values.len() >= pool.max_size {
                    drop(val);
                } else {
                    val.clear();
                    if let Err(val) = pool.values.push(val) {
                        drop(val);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear() {
        let pool = Arc::new(PoolMutex::new());
        let vec = pool.create_with(|| vec![10]);
        drop(vec);
        assert_eq!(pool.len(), 1);
        let vec = pool.create();
        assert!(vec.capacity() == 1 && vec.is_empty());

        let pool = Arc::new(PoolSegQueue::new(1024));
        let vec = pool.create_with(|| vec![10]);
        drop(vec);
        assert_eq!(pool.len(), 1);
        let vec = pool.create();
        assert!(vec.capacity() == 1 && vec.is_empty());

        let pool = Arc::new(PoolArrayQueue::new(1024));
        let vec = pool.create_with(|| vec![10]);
        drop(vec);
        assert_eq!(pool.len(), 1);
        let vec = pool.create();
        assert!(vec.capacity() == 1 && vec.is_empty());
    }
}
