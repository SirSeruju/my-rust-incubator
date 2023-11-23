use chrono::NaiveDate;

fn main() {
    println!("Implemented!");
}

const NOW: &str = "2019-06-26";

struct User {
    birthday: NaiveDate,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        User {
            birthday: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        match NaiveDate::parse_from_str(NOW, "%Y-%m-%d")
            .unwrap()
            .years_since(self.birthday)
        {
            Some(y) => y as u16,
            None => 0,
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
            ((-1, 6, 25), 2020),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            // Should panic cause at least 2016 - 2019 > 1
            // ((2016, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn is_adult() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 25), true),
            ((3000, 6, 27), false),
            ((9999, 6, 27), false),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.is_adult(), expected);
        }
    }

    #[test]
    #[should_panic]
    fn invalid_mounth() {
        let _ = User::with_birthdate(0, 13, 20);
    }
    #[test]
    #[should_panic]
    fn invalid_mounth_zero() {
        let _ = User::with_birthdate(0, 0, 20);
    }

    #[test]
    #[should_panic]
    fn invalid_day() {
        let _ = User::with_birthdate(0, 1, 32);
    }
    #[test]
    #[should_panic]
    fn invalid_day_zero() {
        let _ = User::with_birthdate(0, 1, 0);
    }
}
