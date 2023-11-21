use std::ops::Deref;

struct EmailString(String);

impl TryInto<EmailString> for &str {
    type Error = ();

    fn try_into(self) -> Result<EmailString, Self::Error> {
        // Not a full check, just for test
        let is_email = self.split('@').collect::<Vec<_>>().len() == 2;
        if is_email {
            Ok(EmailString(self.to_string()))
        } else {
            Err(())
        }
    }
}

impl From<EmailString> for String {
    fn from(es: EmailString) -> Self {
        es.0
    }
}

impl Deref for EmailString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<String> for EmailString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

struct Random<T>(T, T, T);

impl<T> From<Random<T>> for (T, T, T) {
    fn from(val: Random<T>) -> (T, T, T) {
        (val.0, val.1, val.2)
    }
}

impl<T> From<(T, T, T)> for Random<T> {
    fn from(val: (T, T, T)) -> Random<T> {
        Random(val.0, val.1, val.2)
    }
}

impl<T> Deref for Random<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let i = rand::random::<u8>() % 3;
        if i == 0 {
            &self.0
        } else if i == 1 {
            &self.1
        } else {
            &self.2
        }
    }
}

#[cfg(test)]
mod test {
    use super::EmailString;
    use super::Random;

    #[test]
    fn email_valid() {
        let e: EmailString = "test@test".try_into().unwrap();
        assert_eq!(e.as_ref(), "test@test");
        let s: String = e.into();
        assert_eq!(s, "test@test".to_string());
    }
    #[test]
    #[should_panic]
    fn email_invalid() {
        let _: EmailString = "test@test@test".try_into().unwrap();
    }
    #[test]
    fn random_value_conversation() {
        let t = (1, 2, 3);
        let r: Random<u8> = t.into();
        let t: (u8, u8, u8) = r.into();
        assert_eq!(t, (1, 2, 3));
    }
    #[test]
    fn random_value_deref() {
        let t: Random<u8> = (1, 2, 3).into();
        let i: &u8 = &t;
        assert_ne!(i, &4);
        assert!(vec![1, 2, 3].contains(i));
    }
}

fn main() {}
