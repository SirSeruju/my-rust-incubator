use std::clone::Clone;
use std::mem;
use std::sync::{Arc, RwLock};

/// Struct implements node cointains value and 2 refences
pub struct DoublyLinkedNode<T> {
    pub prev_node: Option<Arc<RwLock<Box<DoublyLinkedNode<T>>>>>,
    pub next_node: Option<Arc<RwLock<Box<DoublyLinkedNode<T>>>>>,
    pub value: T,
}

/// Implements push left operation, change references and returns created node
pub fn push_left<T>(
    node: &mut Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
    value: T,
) -> Arc<RwLock<Box<DoublyLinkedNode<T>>>> {
    let mut prev_node = None;
    mem::swap(&mut prev_node, &mut node.write().unwrap().prev_node);
    let new_node = Arc::new(RwLock::new(Box::new(DoublyLinkedNode {
        prev_node,
        next_node: Some(node.clone()),
        value,
    })));
    let _ = mem::replace(&mut node.write().unwrap().prev_node, Some(new_node.clone()));
    new_node
}

/// Implements push right operation, change references and returns created node
pub fn push_right<T>(
    node: &mut Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
    value: T,
) -> Arc<RwLock<Box<DoublyLinkedNode<T>>>> {
    let mut next_node = None;
    mem::swap(&mut next_node, &mut node.write().unwrap().next_node);
    let new_node = Arc::new(RwLock::new(Box::new(DoublyLinkedNode {
        prev_node: Some(node.clone()),
        next_node,
        value,
    })));
    let _ = mem::replace(&mut node.write().unwrap().next_node, Some(new_node.clone()));
    new_node
}

/// Disconnect left part(prev reference), returns left part(most right node of part)
pub fn disconnect_left<T>(
    node: &mut Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
) -> Option<Arc<RwLock<Box<DoublyLinkedNode<T>>>>> {
    let mut old_node = None;
    mem::swap(&mut old_node, &mut node.write().unwrap().prev_node);
    match old_node {
        Some(old_node) => {
            old_node.write().unwrap().next_node = None;
            Some(old_node)
        }
        None => None,
    }
}

/// Disconnect right part(next reference), returns right part(most left node of part)
pub fn disconnect_right<T>(
    node: &mut Arc<RwLock<Box<DoublyLinkedNode<T>>>>,
) -> Option<Arc<RwLock<Box<DoublyLinkedNode<T>>>>> {
    let mut old_node = None;
    mem::swap(&mut old_node, &mut node.write().unwrap().next_node);
    match old_node {
        Some(old_node) => {
            old_node.write().unwrap().prev_node = None;
            Some(old_node)
        }
        None => None,
    }
}
