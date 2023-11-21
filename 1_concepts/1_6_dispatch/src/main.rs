use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

mod dynamic_dispatch {
    use super::{Storage, User};

    struct UserRepository {
        storage: dyn Storage<u64, User>,
    }

    impl UserRepository {
        fn set(&mut self, key: u64, val: User) {
            self.storage.set(key, val)
        }
        fn get(&self, key: &u64) -> Option<User> {
            self.storage.get(key).map(|x| (*x).clone())
        }
        fn remove(&mut self, key: &u64) -> Option<User> {
            self.storage.remove(key)
        }
    }
}

mod static_dispatch {
    use super::{Storage, User};

    struct UserRepository<T: Storage<u64, User>> {
        storage: T,
    }

    impl<T: Storage<u64, User>> UserRepository<T> {
        fn set(&mut self, key: u64, val: User) {
            self.storage.set(key, val)
        }
        fn get(&self, key: &u64) -> Option<User> {
            self.storage.get(key).map(|x| (*x).clone())
        }
        fn remove(&mut self, key: &u64) -> Option<User> {
            self.storage.remove(key)
        }
    }
}

fn main() {
    println!("Implement me!");
}
