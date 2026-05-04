/*
 * todo: enforce aliasing rules
 */
mod macros;
mod tests;
pub use crate::list;
mod clone;
mod drop;
mod iterators;
mod algorithms;
mod cursor;

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::FusedIterator,
    marker::PhantomData,
    mem,
    ptr::NonNull,
    usize,
};
//use std::collections::LinkedList

pub struct Node<T> {
    element: T,
    next: Option<NonNull<Self>>,
    prev: Option<NonNull<Self>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    pub fn into_element(self: Box<Self>) -> T {
        self.element
    }

    pub fn element(&self) -> &T {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.element
    }

    pub unsafe fn link(&mut self, right: Option<NonNull<Self>>) {
        self.next = right;
        if let Some(mut right) = right {
            unsafe {
                right.as_mut().prev = Some(NonNull::from_mut(self));
            }
        }
    }

    /*
    pub unsafe fn link_after(
        &mut self,
        node: &mut Node<T>,
    ) -> (Option<NonNull<Node<T>>>, Option<NonNull<Node<T>>>) {
        let next = self.next;
        self.next = Some(NonNull::from_mut(node));
        let prev = node.prev;
        node.prev = Some(NonNull::from_mut(self));
        (prev, next)
    }

    pub unsafe fn link_before(
        &mut self,
        node: &mut Node<T>,
    ) -> (Option<NonNull<Node<T>>>, Option<NonNull<Node<T>>>) {
        let next = self.next;
        self.next = Some(NonNull::from_mut(node));
        let prev = node.prev;
        node.prev = Some(NonNull::from_mut(self));
        (prev, next)
    }
     */
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
    fn ne(&self, other: &Self) -> bool {
        self.element != other.element
    }
}

impl<T: Eq> Eq for Node<T> {}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.element.partial_cmp(&other.element)
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.element.cmp(&other.element)
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    //cap: usize,
    marker: PhantomData<Box<Node<T>>>,
}

// Iterator Impl

// Debug impl
impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

// constructors
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }
}

// private methods
impl<T> LinkedList<T> {
    unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub(crate) unsafe fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        // assert that self contain more then one node
        if self.len <= 1 {
            return;
        }
        let node = unsafe { node.as_mut() };

        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            None => {
                self.head = node.next;
                unsafe {
                    (*self.head.unwrap_unchecked().as_ptr()).prev = None;
                }
            }
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            None => {
                self.tail = node.prev;
                unsafe {
                    (*self.tail.unwrap_unchecked().as_ptr()).next = None;
                }
            }
        };

        self.len -= 1;
    }

    //I still dont understand shit about this

    /*
     *
     *  order:
     *    a   b   c   d   e
     *  0 , 1 , 2 , 3 , 4 , 5
     *
     *  lets say we want to pop d
     *  it's faster to iterate from back of the list
     *  effectively we need to land at 3
     *      len  -  at
     *       5      3
     *
     *
     *
     *
     */
}

//Iterators

//traits
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |elt| self.push_back(elt));
    }
}

// getters
impl<T> LinkedList<T> {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().element) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().element) }
    }
}

// unsafe inserters
// RECHECK
impl<T> LinkedList<T> {
    pub(crate) unsafe fn push_front_ptr(&mut self, elt: T) -> NonNull<Node<T>> {
        let node = Box::new(Node::new(elt));
        let node_ptr = NonNull::from(Box::leak(node));

        unsafe {
            self.push_front_node(node_ptr);
            node_ptr
        }
    }
}

//inserters
// TODO: turn push front mut to use push_front_ptr

// popers

// from constructors
impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(value: [T; N]) -> Self {
        LinkedList::from_iter(value.into_iter())
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        LinkedList::from_iter(value.into_iter())
    }
}

//TODO: see if is it worth to implement a method for slices

// into traits
// TODO: look if this necassry or auto impl by from
impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        Vec::from_iter(self.into_iter())
    }
}

impl<T> LinkedList<T> {
    const fn best_swap_elements() -> bool {
        mem::size_of::<T>() <= mem::size_of::<Option<NonNull<Node<T>>>>() << 1
    }
}

// experemental features
// idea: specialize sorting algorithm for each type
// like when integer use counting sort
// when size is small use array with quicksort
// when array is big use merge sort
impl<T> LinkedList<T> {
    

    //TODO: add remove duplicates
    // TODO: is circular
    pub fn is_circular(&self) -> bool {
        let mut slow = self.iter();
        let mut fast = self.iter();
        //while slow.head.is_some() && fast.head.is_some() {
        //    //if slow.next() == fast.nth(1){
        //    //    return true;
        //    //}
        //}

        return false;
    }
    pub fn to_vec(self) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(self.len);
        for elm in self {
            v.push(elm);
        }
        v
    }
}
// implement cmp for linked list

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other)
    }
    fn ne(&self, other: &Self) -> bool {
        self.len != other.len || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}
impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

//TODO: impelement retain function
