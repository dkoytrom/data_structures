use std::boxed::Box;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct ListNode<T> {
    data: T,
    next: Option<NonNull<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

pub struct Iter<'a, T> {
    head: Option<NonNull<ListNode<T>>>,
    tail: Option<NonNull<ListNode<T>>>,
    len: usize,
    marker: PhantomData<&'a ListNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.data
            })
        }
    }
}

pub struct IterMut<'a, T> {
    head: Option<NonNull<ListNode<T>>>,
    tail: Option<NonNull<ListNode<T>>>,
    len: usize,
    marker: PhantomData<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &mut node.data
            })
        }
    }
}

pub struct List<T: Debug> {
    head: Option<NonNull<ListNode<T>>>,
    tail: Option<NonNull<ListNode<T>>>,
    len: usize,
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let node = Box::new(ListNode::new(data));
        let node_ptr = NonNull::from(Box::leak(node));

        unsafe {
            (*node_ptr.as_ptr()).next = self.head;
            let node = Some(node_ptr);

            if self.head.is_none() {
                self.tail = node;
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<Box<ListNode<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            if self.head.is_none() {
                self.tail = None;
            }

            self.len -= 1;
            node
        })
    }

    pub fn push_back(&mut self, data: T) {
        let node = Box::new(ListNode::new(data));
        let node_ptr = NonNull::from(Box::leak(node));

        unsafe {
            (*node_ptr.as_ptr()).next = None;

            let node_ptr = Some(node_ptr);

            match self.tail {
                None => self.head = node_ptr,
                Some(tail) => (*tail.as_ptr()).next = node_ptr,
            }

            self.tail = node_ptr;
            self.len += 1;
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn print(&self) {
        for i in self.iter() {
            println!("{:?}", i);
        }
    }
}

// DROP TRAIT
impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("Dropping List!");

        while self.pop_front().is_some() {}

        let _ = mem::forget(self);
    }
}
