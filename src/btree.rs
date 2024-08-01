use std::fmt::Debug;
use std::ptr::NonNull;

// BINARY TREE
pub struct BTree<T: Debug> {
    pub root: Option<NonNull<Node<T>>>,
}

impl<T: PartialEq + Debug + PartialOrd> BTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, data: T) -> Option<&Node<T>> {
        if let Some(root) = self.root {
            unsafe {
                return (*root.as_ptr()).search(&data);
            }
        }

        None
    }

    pub fn insert(&mut self, data: T) {
        match self.root {
            Some(root) => unsafe {
                (*root.as_ptr()).insert(data);
            },
            None => {
                let new_node = Box::new(Node::new(data, 1));
                let node_ptr = NonNull::new(Box::leak(new_node));
                self.root = node_ptr;
            }
        }
    }

    pub fn print(&self) {
        if let Some(root) = self.root {
            unsafe {
                (*root.as_ptr()).print();
            }
        }
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    data: T,
    lvl: usize,
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
}

impl<T: PartialOrd + PartialEq + Debug> Node<T> {
    fn new(data: T, lvl: usize) -> Self {
        Self {
            data,
            lvl,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            match self.left {
                None => {
                    let new_node = Box::new(Node::new(data, self.lvl + 1));
                    let node_ptr = NonNull::new(Box::leak(new_node));
                    self.left = node_ptr;
                }
                Some(left) => unsafe {
                    (*left.as_ptr()).insert(data);
                },
            }
        } else {
            match self.right {
                None => {
                    let new_node = Box::new(Node::new(data, self.lvl + 1));
                    let node_ptr = NonNull::new(Box::leak(new_node));
                    self.right = node_ptr;
                }
                Some(right) => unsafe {
                    (*right.as_ptr()).insert(data);
                },
            }
        }
    }

    fn search(&self, data: &T) -> Option<&Node<T>> {
        if *data == self.data {
            return Some(&self);
        } else {
            if let Some(left) = self.left {
                unsafe {
                    let response = (*left.as_ptr()).search(data);

                    if response.is_some() {
                        return response;
                    }
                }
            }

            if let Some(right) = self.right {
                unsafe {
                    let response = (*right.as_ptr()).search(data);

                    if response.is_some() {
                        return response;
                    }
                }
            }
        }

        None
    }

    fn print(&self) {
        if let Some(left) = self.left {
            unsafe {
                (*left.as_ptr()).print();
            }
        }

        println!("{:?}", self.data);

        if let Some(right) = self.right {
            unsafe {
                (*right.as_ptr()).print();
            }
        }
    }
}

// DROP TRAIT
impl<T: Debug> Drop for BTree<T> {
    fn drop(&mut self) {
        println!("Dropping BTree!");

        if self.root.is_none() {
            return;
        }

        let root = self.root.unwrap();

        unsafe {
            let _ = Box::from_raw(root.as_ptr());
        }
    }
}

impl<T: Debug> Drop for Node<T> {
    fn drop(&mut self) {
        if let Some(left) = self.left {
            unsafe {
                let _ = Box::from_raw(left.as_ptr());
            }
        }

        if let Some(right) = self.right {
            unsafe {
                let _ = Box::from_raw(right.as_ptr());
            }
        }
    }
}
