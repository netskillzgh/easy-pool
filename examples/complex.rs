use easy_pool::Clear;

static POOL: easy_pool::Lazy<std::sync::Arc<easy_pool::PoolSegQueue<Test>>> =
    easy_pool::Lazy::new(|| {
        let pool = std::sync::Arc::new(easy_pool::PoolSegQueue::new(1024));
        pool
    });

struct Test {
    pets: Vec<String>,
}

impl Default for Test {
    fn default() -> Self {
        Test {
            pets: Vec::with_capacity(100),
        }
    }
}

impl Clear for Test {
    fn clear(&mut self) {
        self.pets.clear();
    }
}

fn main() {
    assert_eq!(POOL.len(), 0);
    let o = POOL.create_with(|| Test {
        pets: Vec::with_capacity(100),
    });
    assert_eq!(o.pets.capacity(), 100);
    drop(o);
    assert_eq!(POOL.len(), 1);

    let mut objects = Vec::new();
    (0..100).into_iter().for_each(|_| {
        let o = POOL.create();
        objects.push(o);
    });

    drop(objects);

    assert_eq!(POOL.len(), 100);
}
