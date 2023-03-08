use super::List;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

type Next<T> = Rc<RefCell<Node<T>>>;
type Prev<T> = Weak<RefCell<Node<T>>>;

pub(crate) struct Node<T> {
    value: T,
    next: Option<Next<T>>,
    prev: Option<Prev<T>>,
}

pub struct DoublyLinkedList<T> {
    size: usize,
    head: Option<Next<T>>,
    tail: Option<Next<T>>,
}

impl<T> DoublyLinkedList<T> {
    fn get_node(&mut self, idx: usize) -> Option<Next<T>> {
        if idx >= self.size {
            None
        } else if idx > self.size >> 1 {
            let mut ptr = self.tail.clone();
            for _ in idx..(self.size - 1) {
                let prev = ptr.unwrap().borrow().prev.clone();
                ptr = prev.map(|wk| Weak::upgrade(&wk).unwrap());
            }
            ptr
        } else {
            let mut ptr = self.head.clone();
            for _ in 0..idx {
                let next = ptr.unwrap().borrow().next.clone();
                ptr = next;
            }
            ptr
        }
    }
}

impl<T> List<T> for DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push_front(&mut self, value: T) {
        self.size += 1;

        let node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
            prev: None,
        }));

        self.head
            .as_ref()
            .map(|head| head.borrow_mut().prev = Some(Rc::downgrade(&node)));

        if self.size == 1 {
            self.tail = Some(node.clone())
        }
        self.head = Some(node);
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let ret = self.head.clone().unwrap();
        let next = ret.borrow().next.clone();
        self.head = next.map(|x| {
            x.borrow_mut().prev = None;
            x
        });
        if self.size == 0 {
            self.tail = None;
        }

        Rc::try_unwrap(ret).ok().map(|refc| refc.into_inner().value)
    }

    fn push_back(&mut self, value: T) {
        self.size += 1;

        let node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.clone().map(|rc| Rc::downgrade(&rc)),
        }));

        self.tail
            .as_ref()
            .map(|tail| tail.borrow_mut().next = Some(node.clone()));

        if self.size == 1 {
            self.head = Some(node.clone());
        }
        self.tail = Some(node);
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let ret = self.tail.clone().unwrap();
        let prev = ret.borrow().prev.clone();
        self.tail = prev.map(|x| {
            let x = Weak::upgrade(&x).unwrap();
            x.borrow_mut().next = None;
            x
        });
        if self.size == 0 {
            self.head = None;
        }

        Rc::try_unwrap(ret).ok().map(|refc| refc.into_inner().value)
    }

    fn insert(&mut self, idx: usize, value: T) -> Result<(), &str> {
        match idx {
            0 => Ok(self.push_front(value)),
            _ if idx == self.size => Ok(self.push_back(value)),
            _ if idx > self.size => Err("Error: Index out of bounds!"),
            idx => {
                let ptr = self.get_node(idx - 1).unwrap();
                let node = Rc::new(RefCell::new(Node {
                    value,
                    next: ptr.borrow().next.clone(),
                    prev: Some(Rc::downgrade(&ptr)),
                }));
                ptr.borrow()
                    .next
                    .as_ref()
                    .map(|x| x.borrow_mut().prev = Some(Rc::downgrade(&node)));
                ptr.borrow_mut().next = Some(node);
                self.size += 1;
                Ok(())
            }
        }
    }

    fn remove(&mut self, idx: usize) -> Option<T> {
        match idx {
            0 => self.pop_front(),
            _ if idx >= self.size => None,
            _ if idx == self.size - 1 => self.pop_back(),
            idx => {
                self.size -= 1;

                let prev_node = self.get_node(idx - 1).unwrap();
                let ret = prev_node.borrow().next.clone().unwrap();
                let next_node = ret.borrow().next.clone().unwrap();
                next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
                prev_node.borrow_mut().next = Some(next_node);

                Rc::try_unwrap(ret).ok().map(|x| x.into_inner().value)
            }
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(node) = link {
            link = node.borrow_mut().next.take();
        }
    }
}

impl<T: Display> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ptr = self.head.clone();

        if ptr.is_none() {
            return write!(f, "**[Empty]**");
        }

        let mut str = "".to_string();

        while ptr.is_some() {
            str = str + &ptr.clone().unwrap().borrow().value.to_string() + ", ";
            ptr = ptr.unwrap().borrow().next.clone();
        }

        write!(f, "[{}]", str.trim_end_matches(", "))
    }
}

pub struct DoublyLinkedListIter<T>(DoublyLinkedList<T>);

impl<T> Iterator for DoublyLinkedListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = DoublyLinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIter(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_general() {
        assert_eq!("**[Empty]**", DoublyLinkedList::<i32>::new().to_string());
        assert_eq!("[1]", DoublyLinkedList::from_slice(&[1]).to_string());
        assert_eq!(
            "[a, b]",
            DoublyLinkedList::from_slice(&["a", "b"]).to_string()
        );
        assert_eq!(7, DoublyLinkedList::from_slice(&vec![0; 7]).len());
    }

    #[test]
    fn test_head_ops() {
        let mut list = DoublyLinkedList::<i32>::new();

        list.push_front(1);
        assert_eq!("[1]", list.to_string());
        list.push_front(2);
        assert_eq!("[2, 1]", list.to_string());
        list.push_front(3);
        assert_eq!("[3, 2, 1]", list.to_string());

        let mut val = list.pop_front();
        assert_eq!("[2, 1]", list.to_string());
        assert_eq!(Some(3), val);
        val = list.pop_front();
        assert_eq!("[1]", list.to_string());
        assert_eq!(Some(2), val);
        val = list.pop_front();
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(Some(1), val);
        val = list.pop_front();
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(None, val);
    }

    #[test]
    fn test_tail_ops() {
        let mut list = DoublyLinkedList::<i32>::new();

        list.push_back(1);
        assert_eq!("[1]", list.to_string());
        list.push_back(2);
        assert_eq!("[1, 2]", list.to_string());
        list.push_back(3);
        assert_eq!("[1, 2, 3]", list.to_string());

        let mut val = list.pop_back();
        assert_eq!("[1, 2]", list.to_string());
        assert_eq!(Some(3), val);
        val = list.pop_back();
        assert_eq!("[1]", list.to_string());
        assert_eq!(Some(2), val);
        val = list.pop_back();
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(Some(1), val);
        val = list.pop_back();
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(None, val);
    }

    #[test]
    fn test_middle_ops() {
        let mut list = DoublyLinkedList::<i32>::new();

        list.insert(0, 1).unwrap();
        assert_eq!("[1]", list.to_string());
        list.insert(1, 2).unwrap();
        assert_eq!("[1, 2]", list.to_string());
        list.insert(0, 0).unwrap();
        assert_eq!("[0, 1, 2]", list.to_string());
        assert!(list.insert(9, 4).is_err());

        let mut val = list.remove(0);
        assert_eq!("[1, 2]", list.to_string());
        assert_eq!(Some(0), val);
        val = list.remove(1);
        assert_eq!("[1]", list.to_string());
        assert_eq!(Some(2), val);
        val = list.remove(0);
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(Some(1), val);
        val = list.remove(3);
        assert_eq!("**[Empty]**", list.to_string());
        assert_eq!(None, val);
    }

    #[test]
    fn test_iter() {
        let list = DoublyLinkedList::from_slice(&[1, 2, 3, 4, 5]);
        let sum: i32 = list.into_iter().map(|i| i * 2).sum();
        assert_eq!(30, sum);
    }
}
