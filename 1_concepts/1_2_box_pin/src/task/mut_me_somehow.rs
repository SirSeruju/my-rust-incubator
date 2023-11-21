use std::pin::Pin;
use std::rc::Rc;

mod less_generic {
    use super::*;

    trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            // Implementation must be meaningful, and
            // obviously call something requiring `&mut self`.
            // The point here is to practice dealing with
            // `Pin<&mut Self>` -> `&mut self` conversion
            // in different contexts, without introducing
            // any `Unpin` trait bounds.
        }
    }

    // Use Default for garantie something
    impl<T: Default> MutMeSomehow for Box<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let r = Pin::into_inner(self);
            *r = Default::default();
        }
    }

    // Use Default for garantie something
    impl<T: Default> MutMeSomehow for Rc<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let r = Pin::into_inner(self);
            *r = Default::default();
        }
    }

    // Use Default for garantie something
    impl<T: Default> MutMeSomehow for Vec<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            unsafe {
                self.get_unchecked_mut()
                    .iter_mut()
                    .for_each(|x| *x = Default::default());
            }
        }
    }

    impl MutMeSomehow for String {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let r = Pin::into_inner(self);
            *r = r.to_uppercase();
        }
    }

    impl MutMeSomehow for &[u8] {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let r = Pin::into_inner(self);
            *r = &[];
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_all() {
            Pin::new(&mut Box::new(0)).mut_me_somehow();
            Pin::new(&mut Rc::new(0)).mut_me_somehow();
            Pin::new(&mut vec![1, 2, 3]).mut_me_somehow();
            Pin::new(&mut "string".to_string()).mut_me_somehow();
            let mut ns: &[u8] = &[0, 1, 2];
            Pin::new(&mut ns).mut_me_somehow();
        }
    }
}

mod full_generic {
    use super::*;

    trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            // Implementation must be meaningful, and
            // obviously call something requiring `&mut self`.
            // The point here is to practice dealing with
            // `Pin<&mut Self>` -> `&mut self` conversion
            // in different contexts, without introducing
            // any `Unpin` trait bounds.
        }
    }

    // Can't garantie anything about data behind reference
    // so will use Unpin :(
    impl<T: Default + Unpin> MutMeSomehow for T {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            *Pin::into_inner(self) = Default::default();
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_all() {
            Pin::new(&mut Box::new(0)).mut_me_somehow();
            Pin::new(&mut Rc::new(0)).mut_me_somehow();
            Pin::new(&mut vec![1, 2, 3]).mut_me_somehow();
            Pin::new(&mut "string".to_string()).mut_me_somehow();
            let mut ns: &[u8] = &[0, 1, 2];
            Pin::new(&mut ns).mut_me_somehow();
        }
    }
}
