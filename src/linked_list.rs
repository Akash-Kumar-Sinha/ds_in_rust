use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct ListIter<T> {
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { elem, next: None }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn iter<'a>(&'a self) -> ListIter<T> {
        ListIter {
            next: self.head.clone(),
        }
    }

    // Insert at head
    pub fn insert_at_head(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // Insert at tail
    pub fn insert_at_tail(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // Insert at position
    pub fn insert_at_position(&mut self, elem: T, position: i32) {
        if self.head.is_none() || position <= 0 {
            self.insert_at_head(elem);
            return;
        }

        let length = self.get_list_length();

        if position >= length {
            self.insert_at_tail(elem);
            return;
        }

        let new_node = Node::new(elem);
        let mut count = 1;

        let mut prev = self.head.clone();
        let mut current = prev.as_ref().and_then(|node| node.borrow().next.clone());

        while let Some(node) = current {
            if count == position {
                new_node.borrow_mut().next = Some(node.clone());
                prev.unwrap().borrow_mut().next = Some(new_node);
                break;
            }

            prev = Some(node.clone());
            current = node.borrow().next.clone();
            count += 1;
        }
    }

    // Pop from the head
    pub fn pop_at_head(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    // pop from the tail
    pub fn pop_at_tail(&mut self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }

        if self.tail.as_ref().map(|rc| Rc::as_ptr(rc))
            == self.head.as_ref().map(|rc| Rc::as_ptr(rc))
        {
            return self.pop_at_head();
        }

        let mut prev = None;
        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().next.as_ref().map(|rc| Rc::as_ptr(rc))
                == self.tail.as_ref().map(|rc| Rc::as_ptr(rc))
            {
                prev = Some(node.clone());
                break;
            }
            current = node.borrow().next.clone();
        }
        let old_tail = self.tail.take(); // Take the tail first

        if let Some(prev_node) = prev {
            prev_node.borrow_mut().next = None; // Remove the last node
            self.tail = Some(prev_node); // Update tail
        } else {
            self.head = None;
            self.tail = None;
        }

        old_tail.map(|new_tail| Rc::try_unwrap(new_tail).ok().unwrap().into_inner().elem)
    }

    // pop from position
    pub fn pop_at_position(&mut self, position: i32) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if position == 0 {
            return self.pop_at_head();
        }

        if self.head.as_ref().map(|value| Rc::as_ptr(value))
            == self.tail.as_ref().map(|value| Rc::as_ptr(value))
        {
            return self.pop_at_head();
        }

        let mut count = 1;

        let mut prev = self.head.clone();
        let mut current = prev.as_ref().and_then(|node| node.borrow().next.clone());

        while let Some(node) = current {
            if count == position {
                prev.as_ref()?.borrow_mut().next = node.borrow_mut().next.take();

                if prev.as_ref()?.borrow().next.is_none() {
                    self.tail = prev.clone();
                }

                return Some(Rc::try_unwrap(node).ok()?.into_inner().elem);
            }

            prev = Some(node.clone());
            current = node.borrow().next.clone();
            count += 1;
        }

        None
    }

    pub fn peak_elem(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn last_elem(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn get_list_length(&self) -> i32 {
        let mut count = 0;
        if self.head.is_none() {
            return count;
        }

        let mut current = self.head.clone();

        while let Some(node) = current {
            count += 1;
            current = node.borrow().next.clone();
        }

        count
    }
}

impl<T: Clone> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let node_ref = node.borrow();
            self.next = node_ref.next.clone();
            node_ref.elem.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_new_list() {
        let list: List<i32> = List::new();
        assert_eq!(list.get_list_length(), 0);
        assert!(list.peak_elem().is_none());
        assert!(list.last_elem().is_none());
    }

    #[test]
    fn test_insert_at_head() {
        let mut list = List::new();
        list.insert_at_head(10);
        list.insert_at_head(20);
        list.insert_at_head(30);
        assert_eq!(list.get_list_length(), 3);
        assert_eq!(list.pop_at_head(), Some(30));
        assert_eq!(list.pop_at_head(), Some(20));
        assert_eq!(list.pop_at_head(), Some(10));
    }

    #[test]
    fn test_insert_at_tail() {
        let mut list = List::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        assert_eq!(list.get_list_length(), 3);
        assert_eq!(list.pop_at_tail(), Some(3));
        assert_eq!(list.pop_at_tail(), Some(2));
        assert_eq!(list.pop_at_tail(), Some(1));
    }

    #[test]
    fn test_insert_at_position() {
        let mut list = List::new();
        list.insert_at_position(1, 0);
        list.insert_at_position(3, 1);
        list.insert_at_position(2, 1);
        list.insert_at_position(0, 0);
        assert_eq!(list.get_list_length(), 4);
        assert_eq!(list.pop_at_head(), Some(0));
        assert_eq!(list.pop_at_head(), Some(1));
        assert_eq!(list.pop_at_head(), Some(2));
        assert_eq!(list.pop_at_head(), Some(3));
    }

    #[test]
    fn test_pop_at_head() {
        let mut list = List::new();
        list.insert_at_head(5);
        assert_eq!(list.pop_at_head(), Some(5));
        assert_eq!(list.pop_at_head(), None);
    }

    #[test]
    fn test_pop_at_tail() {
        let mut list = List::new();
        list.insert_at_tail(5);
        assert_eq!(list.pop_at_tail(), Some(5));
        assert_eq!(list.pop_at_tail(), None);
    }

    #[test]
    fn test_peak_elem() {
        let mut list = List::new();
        assert!(list.peak_elem().is_none());
        list.insert_at_head(42);
        assert_eq!(list.peak_elem().map(|v| *v), Some(42));
    }

    #[test]
    fn test_last_elem() {
        let mut list = List::new();
        assert!(list.last_elem().is_none());
        list.insert_at_tail(99);
        assert_eq!(list.last_elem().map(|v| *v), Some(99));
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_pop_at_position() {
        let mut list = List::new();

        assert_eq!(list.pop_at_position(0), None);

        list.insert_at_tail(10);
        list.insert_at_tail(20);
        list.insert_at_tail(30);
        list.insert_at_tail(40);
        list.insert_at_tail(50);

        assert_eq!(list.pop_at_position(2), Some(30));

        assert_eq!(list.pop_at_position(0), Some(10));

        assert_eq!(list.pop_at_position(2), Some(50));

        assert_eq!(list.get_list_length(), 2);
        assert_eq!(list.pop_at_head(), Some(20));
        assert_eq!(list.pop_at_head(), Some(40));

        assert_eq!(list.pop_at_head(), None);
    }

}