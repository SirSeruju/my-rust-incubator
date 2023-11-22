use std::clone::Clone;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};

use super::doubly_linked_node::{
    disconnect_left, disconnect_right, push_left, push_right, DoublyLinkedNode,
};

/// Helper for store nodes with implemented semantic in `DoublyLinkedList`:
/// If no nodes => Nothing; One => fields references to this one node;
/// Otherwise stores references to the most left and most right nodes
enum DoublyLinkedNodes<T> {
    Nothing,
    Something {
        first_node: Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
        last_node: Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
        length: usize,
    },
}

/// Thread safe doubly linked list
pub struct DoublyLinkedList<T>(Arc<Mutex<DoublyLinkedNodes<T>>>);

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList(Arc::new(Mutex::new(DoublyLinkedNodes::Nothing)))
    }

    pub fn push_left(&self, value: T) {
        let mut dll = self.0.lock().unwrap();
        match *dll {
            DoublyLinkedNodes::Nothing => {
                let node = Arc::new(RwLock::new(Box::new(DoublyLinkedNode {
                    prev_node: None,
                    next_node: None,
                    value,
                })));
                *dll = DoublyLinkedNodes::Something {
                    first_node: node.clone(),
                    last_node: node,
                    length: 1,
                };
            }
            DoublyLinkedNodes::Something {
                ref mut first_node,
                last_node: _,
                ref mut length,
            } => {
                *length += 1;
                *first_node = push_left(first_node, value);
            }
        }
    }
    pub fn push_right(&self, value: T) {
        let mut dll = self.0.lock().unwrap();
        match *dll {
            DoublyLinkedNodes::Nothing => {
                let node = Arc::new(RwLock::new(Box::new(DoublyLinkedNode {
                    prev_node: None,
                    next_node: None,
                    value,
                })));
                *dll = DoublyLinkedNodes::Something {
                    first_node: node.clone(),
                    last_node: node,
                    length: 1,
                };
            }
            DoublyLinkedNodes::Something {
                first_node: _,
                ref mut last_node,
                ref mut length,
            } => {
                *length += 1;
                *last_node = push_right(last_node, value);
            }
        }
    }
    pub fn pop_left(&self) -> Option<T> {
        let mut dll = self.0.lock().unwrap();
        match *dll {
            DoublyLinkedNodes::Nothing => None,
            DoublyLinkedNodes::Something {
                ref mut first_node,
                last_node: _,
                ref mut length,
            } => {
                if *length == 1 {
                    let node = first_node.clone();
                    *dll = DoublyLinkedNodes::Nothing;
                    match Arc::try_unwrap(node) {
                        Ok(v) => Some(v.into_inner().unwrap().value),
                        Err(_) => panic!("misuse, cross references!"),
                    }
                } else {
                    *length -= 1;
                    let node = first_node.clone();
                    *first_node = disconnect_right(first_node).unwrap();
                    match Arc::try_unwrap(node) {
                        Ok(v) => Some(v.into_inner().unwrap().value),
                        Err(_) => panic!("misuse, cross references!"),
                    }
                }
            }
        }
    }
    pub fn pop_right(&self) -> Option<T> {
        let mut dll = self.0.lock().unwrap();
        match *dll {
            DoublyLinkedNodes::Nothing => None,
            DoublyLinkedNodes::Something {
                first_node: _,
                ref mut last_node,
                ref mut length,
            } => {
                if *length == 1 {
                    let node = last_node.clone();
                    *dll = DoublyLinkedNodes::Nothing;
                    match Arc::try_unwrap(node) {
                        Ok(v) => Some(v.into_inner().unwrap().value),
                        Err(_) => panic!("misuse, cross references!"),
                    }
                } else {
                    *length -= 1;
                    let node = last_node.clone();
                    *last_node = disconnect_left(last_node).unwrap();
                    match Arc::try_unwrap(node) {
                        Ok(v) => Some(v.into_inner().unwrap().value),
                        Err(_) => panic!("misuse, cross references!"),
                    }
                }
            }
        }
    }
}

impl<T> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        DoublyLinkedList(Arc::clone(&self.0))
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for DoublyLinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_left()
    }
}

impl<V> FromIterator<V> for DoublyLinkedList<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let dle: Self = Default::default();
        for v in iter {
            dle.push_right(v)
        }
        dle
    }
}

impl<T: fmt::Debug> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DoublyLinkedList ")?;
        let mut f = f.debug_list();
        let mut dll = self.0.lock().unwrap();
        match *dll {
            DoublyLinkedNodes::Nothing => (),
            DoublyLinkedNodes::Something {
                ref mut first_node,
                last_node: _,
                ref mut length,
            } => {
                let mut node = first_node.clone();
                for _ in 0..*length - 1 {
                    f.entry(&node.clone().read().unwrap().value);
                    node = node.clone().read().unwrap().next_node.clone().unwrap();
                }
                f.entry(&node.clone().read().unwrap().value);
            }
        }
        f.finish()
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn format() {
        let d: DoublyLinkedList<u8> = DoublyLinkedList::new();
        assert_eq!("DoublyLinkedList []", format!("{:?}", d));
        d.push_right(0);
        assert_eq!("DoublyLinkedList [0]", format!("{:?}", d));
        d.push_right(1);
        assert_eq!("DoublyLinkedList [0, 1]", format!("{:?}", d));
    }

    #[test]
    fn iters() {
        let d = vec![1, 2, 3, 4]
            .into_iter()
            .collect::<DoublyLinkedList<u8>>();
        assert_eq!(vec![1, 2, 3, 4], d.into_iter().collect::<Vec<u8>>());
    }

    #[test]
    fn threads() {
        let dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let thread1;
        let thread2;

        {
            let dll = dll.clone();
            thread1 = thread::spawn(move || {
                for i in 0..10 {
                    dll.push_right(i);
                }
            });
        }
        {
            let dll = dll.clone();
            thread2 = thread::spawn(move || {
                for i in 0..10 {
                    dll.push_left(i);
                }
            });
        }

        thread1.join().unwrap();
        thread2.join().unwrap();

        assert_eq!(dll.into_iter().count(), 20);
    }
}
