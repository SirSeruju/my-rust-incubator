trait User<'a, Id, Nickname> {
    fn get_id(&'a self) -> &'a Id;
    fn get_nickname(&'a self) -> &'a Nickname;
}

trait UsersRepository<'a, Id, Nickname, U>
where
    U: User<'a, Id, Nickname>,
{
    fn user_by_id(self, id: &Id) -> Option<&'a U>;
    fn users_by_ids(self, ids: &[&Id]) -> Vec<Option<&'a U>>;
    fn users_by_nickname(self, nickname: &'a Nickname) -> Vec<&'a Id>;
}

use im::HashMap;
use std::hash::Hash;

impl<'a, Id, Nickname, U> UsersRepository<'a, Id, Nickname, U> for &'a HashMap<Id, U>
where
    Id: Eq + Hash,
    Nickname: ToString,
    U: User<'a, Id, Nickname>,
{
    fn user_by_id(self, id: &Id) -> Option<&'a U> {
        self.get(id)
    }

    fn users_by_ids(self, ids: &[&Id]) -> Vec<Option<&'a U>> {
        ids.iter().map(|id| self.get(id)).collect()
    }

    fn users_by_nickname(self, nickname: &'a Nickname) -> Vec<&'a Id> {
        self.values()
            .filter(|user| {
                user.get_nickname()
                    .to_string()
                    .contains(&nickname.to_string())
            })
            .map(|user| user.get_id())
            .collect()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct SpecUser<Id, Nickname> {
    id: Id,
    nickname: Nickname,
}

impl<Id, Nickname> SpecUser<Id, Nickname> {
    fn new(id: Id, nickname: Nickname) -> Self {
        SpecUser { id, nickname }
    }
}

impl<'a, Id, Nickname> User<'a, Id, Nickname> for SpecUser<Id, Nickname> {
    fn get_id(&'a self) -> &'a Id {
        &self.id
    }

    fn get_nickname(&'a self) -> &'a Nickname {
        &self.nickname
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn user_by_id() {
        let mut ur = HashMap::new();
        let u = SpecUser::new(0, "u0".to_string());
        ur.insert(u.get_id().clone(), u);
        let u = SpecUser::new(1, "u1".to_string());
        ur.insert(u.get_id().clone(), u);
        assert_eq!(&"u0".to_string(), ur.user_by_id(&0).unwrap().get_nickname());
        assert_eq!(&"u1".to_string(), ur.user_by_id(&1).unwrap().get_nickname());
        assert_eq!(None, ur.user_by_id(&2));
    }
    #[test]
    fn users_by_ids() {
        let mut ur = HashMap::new();
        let u = SpecUser::new(0, "u0".to_string());
        ur.insert(u.get_id().clone(), u);
        let u = SpecUser::new(1, "u1".to_string());
        ur.insert(u.get_id().clone(), u);
        assert_eq!(
            vec![None, Some(&SpecUser::new(0, "u0".to_string()))],
            ur.users_by_ids(&vec![&2, &0])
        );
    }
    #[test]
    fn users_by_nickname() {
        let mut ur = HashMap::new();
        let u = SpecUser::new(0, "test_u0".to_string());
        ur.insert(u.get_id().clone(), u);
        let u = SpecUser::new(1, "u1_test".to_string());
        ur.insert(u.get_id().clone(), u);
        let u = SpecUser::new(2, "a1".to_string());
        ur.insert(u.get_id().clone(), u);
        assert_eq!(vec![&0, &1], ur.users_by_nickname(&"u".to_string()));
    }
}

fn main() {}
