use std::borrow::Cow;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserRepository {
    fn set(&self, key: u64, val: User);
    fn get(&self, key: &u64) -> Option<User>;
    fn remove(&self, key: &u64) -> Option<User>;
}

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

struct CreateUser;
impl Command for CreateUser {}

#[derive(PartialEq, Eq, Debug)]
enum UserError {
    AlreadyExist,
}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;

    fn handle_command(&self, _cmd: &CreateUser, ctx: &Self::Context) -> Self::Result {
        match ctx.get(&self.id) {
            Some(_) => Err(UserError::AlreadyExist),
            None => {
                ctx.set(self.id, self.clone());
                Ok(())
            }
        }
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct Repo {
        users: RefCell<HashMap<u64, User>>,
    }

    impl Repo {
        fn new() -> Repo {
            Repo {
                users: RefCell::new(HashMap::new()),
            }
        }
    }

    impl UserRepository for Repo {
        fn set(&self, key: u64, val: User) {
            self.users.borrow_mut().insert(key, val);
        }
        fn get(&self, key: &u64) -> Option<User> {
            self.users.borrow().get(key).map(|x| (*x).clone())
        }
        fn remove(&self, key: &u64) -> Option<User> {
            self.users.borrow_mut().remove(key)
        }
    }

    #[test]
    fn test_users() {
        let r = Repo::new();
        let u = User {
            id: 0,
            email: "email@mail.mail".into(),
            activated: true,
        };
        assert_eq!(u.handle_command(&CreateUser, &r), Ok(()));
        assert_eq!(
            u.handle_command(&CreateUser, &r),
            Err(UserError::AlreadyExist)
        );
        let gu = r.get(&u.id).unwrap();
        assert_eq!(gu, u);
    }
}
