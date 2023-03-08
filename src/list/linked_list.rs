use super::List;
use std::fmt::Display;

// #[derive(Clone)]
pub(crate) struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn get_node(&mut self, idx: usize) -> Option<&mut Node<T>> {
        if idx >= self.size {
            return None;
        }

        let mut ptr = self.head.as_deref_mut();
        for _ in 0..idx {
            ptr = ptr.unwrap().next.as_deref_mut();
        }

        ptr
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        // if idx >= self.size {
        //     return None;
        // }

        // let mut ptr = self.head.as_mut();
        // for _ in 0..idx {
        //     ptr = ptr.unwrap().next.as_mut();
        // }
        // Some(&mut ptr.unwrap().value)

        self.get_node(idx).map(|node| &mut node.value)
    }

    pub fn get(&mut self, idx: usize) -> Option<&T> {
        self.get_node(idx).map(|node| &node.value)
    }
}

impl<T> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push_front(&mut self, value: T) {
        self.size += 1;
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    fn pop_front(&mut self) -> Option<T> {
        if let Some(x) = self.head.take() {
            self.size -= 1;
            self.head = x.next;
            Some(x.value)
        } else {
            None
        }
    }

    fn push_back(&mut self, value: T) {
        if self.head.is_none() {
            return self.push_front(value);
        }

        // let mut ptr = self.head.as_mut();
        // let tail = loop {
        //     match ptr {
        //         Some(not_tail) if not_tail.next.is_some() => ptr = not_tail.next.as_mut(),
        //         Some(tail) => break tail,
        //         None => unreachable!(),
        //     }
        // };

        // tail.next = Some(Box::new(Node { value, next: None }));
        // self.size += 1;

        let tail = self.get_node(self.size - 1).unwrap();
        tail.next = Some(Box::new(Node { value, next: None }));
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.size {
            0..=1 => {
                self.size = 0;
                self.head.take().map(|node| node.value)
            }
            _ => {
                // let mut ptr = self.head.as_mut();
                // let tail = loop {
                //     match ptr {
                //         Some(x) if x.next.is_some() && x.next.as_ref().unwrap().next.is_some() => {
                //             // Get tail's prev-node
                //             ptr = x.next.as_mut()
                //         }
                //         Some(tail) => break tail,
                //         None => unreachable!(),
                //     }
                // };

                // self.size -= 1;
                // tail.next.take().map(|node| node.value)

                self.size -= 1;

                // New-tail is tail's prev-node
                let new_tail = self.get_node(self.size - 1).unwrap();
                new_tail.next.take().map(|node| node.value)
            }
        }
    }

    fn insert(&mut self, idx: usize, value: T) -> Result<(), &str> {
        match idx {
            0 => Ok(self.push_front(value)),
            _ if idx == self.size => Ok(self.push_back(value)),
            _ if idx > self.size => Err("Error: Index out of bounds!"),
            idx => {
                let ptr = self.get_node(idx - 1).unwrap();
                let node = Node {
                    value,
                    next: ptr.next.take(),
                };
                ptr.next = Some(Box::new(node));
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

                let ptr = self.get_node(idx - 1).unwrap();
                let ret = ptr.next.take().unwrap();
                ptr.next = ret.next;
                Some(ret.value)
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ptr = self.head.as_ref();

        if ptr.is_none() {
            return write!(f, "**[Empty]**");
        }

        let mut str = "".to_string();

        while ptr.is_some() {
            str = str + &ptr.unwrap().value.to_string() + ", ";
            ptr = ptr.unwrap().next.as_ref();
        }

        write!(f, "[{}]", str.trim_end_matches(", "))
    }
}

pub struct LinkedListIter<T>(LinkedList<T>);

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_general() {
        assert_eq!("**[Empty]**", LinkedList::<i32>::new().to_string());
        assert_eq!("[1]", LinkedList::from_slice(&[1]).to_string());
        assert_eq!("[a, b]", LinkedList::from_slice(&["a", "b"]).to_string());
        assert_eq!(7, LinkedList::from_slice(&vec![0; 7]).len());
    }

    #[test]
    fn test_head_ops() {
        let mut list = LinkedList::<i32>::new();

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
        let mut list = LinkedList::<i32>::new();

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
        let mut list = LinkedList::<i32>::new();

        list.insert(0, 1).unwrap();
        assert_eq!("[1]", list.to_string());
        list.insert(1, 2).unwrap();
        assert_eq!("[1, 2]", list.to_string());
        list.insert(0, 3).unwrap();
        assert_eq!("[3, 1, 2]", list.to_string());
        assert!(list.insert(9, 4).is_err());

        let val = list.get_mut(3);
        assert_eq!(None, val);
        let val = list.get(1);
        assert_eq!(Some(&1), val);
        let val = list.get_mut(0);
        assert_eq!(Some(&mut 3), val);
        *val.unwrap() = 0;
        assert_eq!("[0, 1, 2]", list.to_string());

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
        let list = LinkedList::from_slice(&[1, 2, 3, 4, 5]);
        let sum: i32 = list.into_iter().map(|i| i * 2).sum();
        assert_eq!(30, sum);
    }
}
