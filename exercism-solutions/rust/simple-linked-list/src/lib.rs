// use std::{default, iter::FromIterator, mem::take};

// pub struct Node<T> {
//     next: MaybeNode<T>,
//     value: T,
// }

// type MaybeNode<T> = Option<Box<Node<T>>>;

// #[derive(Default)]
// pub struct SimpleLinkedList<T> {
//     head: MaybeNode<T>,
// }

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         Self { head: None }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.head.is_none()
//     }

//     pub fn len(&self) -> usize {
//         // We could've stored a field `total_nodes`, but I don't want to
//         // do it here :)
//         let mut curr = &self.head;
//         let mut length: usize = 0;
//         while curr.is_some() {
//             curr = &curr.as_ref().unwrap().next;
//             length += 1;
//         }

//         length
//     }

//     /// Returns a mutable reference to the last node with a `None` next field.
//     fn last_mut(&mut self) -> &mut MaybeNode<T> {
//         if self.head.is_none() {
//             return &mut self.head;
//         }

//         let mut curr = &mut self.head;
//         while curr.as_mut().unwrap().next.is_some() {
//             curr = &mut curr.as_mut().unwrap().next;
//         }

//         curr
//     }

//     fn last(&self) -> &MaybeNode<T> {
//         if self.head.is_none() {
//             return &self.head;
//         }

//         let mut curr = &self.head;
//         while curr.as_ref().unwrap().next.is_some() {
//             curr = &curr.as_ref().unwrap().next;
//         }

//         curr
//     }

//     pub fn push(&mut self, _element: T) {
//         let last = self.last_mut();
//         if last.is_none() {
//             *last = Some(Box::new(Node {
//                 next: None,
//                 value: _element,
//             }));
//         } else {
//             let last = last.as_mut().unwrap();
//             last.next = Some(Box::new(Node {
//                 next: None,
//                 value: _element,
//             }));
//         }
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         let last = self.last_mut();
//         if last.is_none() {
//             return None;
//         }

//         let last = last.take().unwrap();
//         Some(last.value)
//     }

//     pub fn peek(&self) -> Option<&T> {
//         let last = self.last();
//         if last.is_none() {
//             return None;
//         }

//         let last = last.as_ref().unwrap();
//         Some(&last.value)
//     }

//     #[must_use]
//     pub fn rev(self) -> SimpleLinkedList<T> {
//         let elements: Vec<T> = self.into();
//         elements.into_iter().rev().collect()
//     }
// }

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
//         let mut list = Self::new();

//         for item in _iter {
//             list.push(item);
//         }

//         list
//     }
// }

// impl<T> From<SimpleLinkedList<T>> for Vec<T> {
//     fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
//         let mut vec = Vec::new();
//         while let Some(item) = _linked_list.pop() {
//             vec.push(item);
//         }

//         vec.reverse();
//         vec
//     }
// }

use std::iter::FromIterator;

pub struct Node<T> {
    next: MaybeNode<T>,
    value: T,
}

type MaybeNode<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(value: T, next: MaybeNode<T>) -> Self {
        Self { next, value }
    }
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: MaybeNode<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        // We could've stored a field `total_nodes`, but I don't want to
        // do it here :)
        let mut curr = &self.head;
        let mut length: usize = 0;
        while curr.is_some() {
            curr = &curr.as_ref().unwrap().next;
            length += 1;
        }

        length
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let elements: Vec<T> = self.into();
        elements.into_iter().rev().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();

        for item in _iter {
            list.push(item);
        }

        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(item) = _linked_list.pop() {
            vec.push(item);
        }

        vec.reverse();
        vec
    }
}
