use std::sync::Arc;

use easy_pool::{Clear, PoolMutex};
use easy_pool_proc_macro::EasyPoolMutex;

// It will create the pool and create the functions T::create_with & T::create.
// This is optional but you have to create the pool yourself.
// Like this : let pool = Arc::new(PoolMutex::with_config(1024, 1024));.
#[derive(EasyPoolMutex)]
struct Test {
    age: u8,
    pets: Vec<String>,
}

impl Default for Test {
    fn default() -> Self {
        Test {
            age: 0,
            pets: vec![],
        }
    }
}

impl Clear for Test {
    fn clear(&mut self) {
        self.age = 0;
        self.pets.clear();
    }
}

fn main() {
    // Easiest way.
    let mut test = Test::create_with(|| Test {
        age: 15,
        pets: vec!["cat".to_string()],
    });
    assert_eq!(test.age, 15);
    test.age = 10;
    assert_eq!(test.age, 10);
    let test = Test::create();
    assert_eq!(test.age, 0);

    // Or more complex.
    let pool = Arc::new(PoolMutex::with_config(1024, 1024));
    let result = pool.create_with(|| Test {
        age: 15,
        pets: vec!["cat".to_string()],
    });
    assert_eq!(result.age, 15);
}
