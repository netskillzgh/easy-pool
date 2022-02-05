```toml
[dependencies]
easy-pool = "0.1.0"
```

Pools : 

- [PoolArrayQueue](https://docs.rs/easy-pool/0.1.0/easy_pool/pool_array_queue/struct.PoolArrayQueue.html)
- [PoolMutex](https://docs.rs/easy-pool/0.1.0/easy_pool/pool_mutex/struct.PoolMutex.html)
- [PoolSegQueue](https://docs.rs/easy-pool/0.1.0/easy_pool/pool_seg_queue/struct.PoolSegQueue.html)

<hr>

## Simple example

```rust, no_run
use easy_pool::PoolMutex;
use std::sync::Arc;

let pool = Arc::new(PoolMutex::<Vec<u8>>::new());
let val = pool.create_with(|| Vec::with_capacity(1024));
drop(val);
// return to the pool
```
