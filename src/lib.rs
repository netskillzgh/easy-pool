#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![doc = include_str!("../README.md")]

use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::Arc,
};

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
    inner: ManuallyDrop<T>,
    ref_pool: PoolType<T>,
}

impl<T: Clear> PoolObjectContainer<T> {
    fn new(val: T, ref_pool: PoolType<T>) -> Self {
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
