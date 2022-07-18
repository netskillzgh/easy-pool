use std::sync::Arc;

use easy_pool::{Clear, PoolMutex};
use easy_pool_proc_macro::EasyPoolMutex;

// It will create the pool and create the functions T::create_with & T::create.
// This derive is optional but you have to create the pool yourself.
// Like this : let pool = Arc::new(PoolMutex::with_config(1024, 1024));.
#[derive(EasyPoolMutex, Default)]
struct Test {
    pets: Vec<String>,
}

impl Clear for Test {
    fn clear(&mut self) {
        self.pets.clear();
    }
}

fn main() {
    // Easiest way.
    let mut test = Test::create_with(|| Test {
        pets: Vec::with_capacity(100),
    });
    assert_eq!(test.pets.capacity(), 100);
    test.pets.push("Cat".to_string());
    assert_eq!(test.pets.first().unwrap(), "Cat");
    test.pets.extend(vec!["Dog".to_string(); 100]);
    assert_eq!(test.pets.len(), 101);
    assert_eq!(test.pets.capacity(), 200);
    drop(test);

    // The function create will reuse the old "test".
    let test = Test::create_with(|| Test {
        pets: Vec::with_capacity(100),
    });
    assert_eq!(test.pets.len(), 0);
    assert_eq!(test.pets.capacity(), 200);

    // Or more complex.
    let pool = Arc::new(PoolMutex::with_config(1024, 1024));
    let result = pool.create_with(|| Test {
        pets: Vec::with_capacity(100),
    });
    assert_eq!(result.pets.capacity(), 100);
}
