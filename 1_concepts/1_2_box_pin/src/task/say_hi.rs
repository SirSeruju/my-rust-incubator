use std::fmt;
use std::pin::Pin;
use std::rc::Rc;

mod less_generic {
    use super::*;

    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: std::fmt::Debug> SayHi for Box<T> {
        fn say_hi(self: Pin<&Box<T>>) {
            println!("Hi from Box {{ {:?} }}", Pin::into_inner(self))
        }
    }

    impl<T: std::fmt::Debug> SayHi for Rc<T> {
        fn say_hi(self: Pin<&Rc<T>>) {
            println!("Hi from Rc {{ {:?} }}", Pin::into_inner(self))
        }
    }

    impl<T: std::fmt::Debug> SayHi for Vec<T> {
        fn say_hi(self: Pin<&Vec<T>>) {
            unsafe {
                println!("Hi from Vec {{ {:?} }}", Pin::into_inner_unchecked(self));
            }
        }
    }

    impl SayHi for String {
        fn say_hi(self: Pin<&String>) {
            println!("Hi from String {{ {:?} }}", Pin::into_inner(self))
        }
    }

    impl SayHi for &[u8] {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from &[u8] {{ {:?} }}", Pin::into_inner(self))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_all() {
            Pin::new(&Box::new(0)).say_hi();
            Pin::new(&Rc::new(0)).say_hi();
            Pin::new(&vec![1, 2, 3]).say_hi();
            Pin::new(&"string".to_string()).say_hi();
            let ns: &[u8] = &[0, 1, 2];
            Pin::new(&ns).say_hi();
        }
    }
}

mod full_generic {
    use super::*;
    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: std::fmt::Debug> SayHi for T {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {} {{ {:?} }}", std::any::type_name::<T>(), self)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_all() {
            Pin::new(&Box::new(0)).say_hi();
            Pin::new(&Rc::new(0)).say_hi();
            Pin::new(&vec![1, 2, 3]).say_hi();
            Pin::new(&"string".to_string()).say_hi();
            let ns: &[u8] = &[0, 1, 2];
            Pin::new(&ns).say_hi();
        }
    }
}
