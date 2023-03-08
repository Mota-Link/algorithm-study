use std::{collections::VecDeque, fmt::Display};

pub(crate) struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Option<Box<Self>> {
        Some(Box::new(Node {
            key,
            value,
            left: None,
            right: None,
        }))
    }

    fn lpeek(&self) -> Option<&K> {
        self.left.as_deref().map(|x| &x.key)
    }

    fn rpeek(&self) -> Option<&K> {
        self.right.as_deref().map(|x| &x.key)
    }

    fn child_state(&self) -> u8 {
        match (self.lpeek(), self.rpeek()) {
            (None, None) => 0b00,
            (None, Some(_)) => 0b01,
            (Some(_), None) => 0b10,
            (Some(_), Some(_)) => 0b11,
        }
    }
}

pub struct BSTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> BSTree<K, V> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    pub fn get(&self, key: K) -> Option<V>
    where
        K: Ord,
        V: Clone,
    {
        if self.root.is_none() {
            return None;
        }

        let mut current = self.root.as_ref().unwrap();

        while key != current.key {
            if key > current.key && current.rpeek().is_some() {
                current = current.right.as_ref().unwrap()
            } else if key < current.key && current.lpeek().is_some() {
                current = current.left.as_ref().unwrap()
            } else {
                return None;
            }
        }

        Some(current.value.clone())
    }

    pub fn insert(&mut self, key: K, value: V)
    where
        K: Ord,
    {
        fn recursion_insert<K: Ord, V>(tree: Option<&mut Box<Node<K, V>>>, key: K, value: V) {
            let node = tree.unwrap();
            if key < node.key {
                if node.left.is_none() {
                    node.left = Node::new(key, value);
                    return;
                }
                recursion_insert(node.left.as_mut(), key, value)
            } else {
                if node.right.is_none() {
                    node.right = Node::new(key, value);
                    return;
                }
                recursion_insert(node.right.as_mut(), key, value)
            }
        }

        if self.root.is_none() {
            self.root = Node::new(key, value);
            return;
        }
        recursion_insert(self.root.as_mut(), key, value)
    }

    pub fn remove(&mut self, key: K) -> Result<V, ()>
    where
        K: Ord,
        V: Clone,
    {
        if self.root.is_none() {
            return Err(());
        }

        let mut current = self.root.as_mut().unwrap();
        loop {
            match key.cmp(&current.key) {
                std::cmp::Ordering::Less => match current.lpeek() {
                    None => return Err(()),
                    Some(x) if x != &key => {
                        current = current.left.as_mut().unwrap();
                        continue;
                    }
                    Some(_) => {
                        return match current.left.as_ref().unwrap().child_state() {
                            0 => Ok(current.left.take().unwrap().value),
                            1 => {
                                let ret = current.left.take().unwrap();
                                current.left = ret.right;
                                Ok(ret.value)
                            }
                            2 => {
                                let ret = current.left.take().unwrap();
                                current.left = ret.left;
                                Ok(ret.value)
                            }
                            _ => {
                                current = current.left.as_mut().unwrap();
                                let ret = current.value.clone();
                                let mut temp: &mut Box<Node<K, V>> = current;
                                while temp.right.as_ref().unwrap().rpeek().is_some() {
                                    temp = temp.right.as_mut().unwrap();
                                }
                                let ltree_biggest = temp.right.take().unwrap();
                                temp.right = ltree_biggest.left;
                                current.key = ltree_biggest.key;
                                current.value = ltree_biggest.value;

                                Ok(ret)
                            }
                        }
                    }
                },
                std::cmp::Ordering::Equal => {
                    return match current.child_state() {
                        0 => Ok(self.root.take().unwrap().value),
                        1 => {
                            let ret = self.root.take().unwrap();
                            self.root = ret.right;
                            Ok(ret.value)
                        }
                        2 => {
                            let ret = self.root.take().unwrap();
                            self.root = ret.left;
                            Ok(ret.value)
                        }
                        _ => {
                            let mut temp = current.left.as_mut().unwrap();
                            let ret = current.value.clone();
                            if temp.rpeek().is_none() {
                                let mut ltree_biggest = current.left.take().unwrap();
                                current.left = ltree_biggest.left.take();
                                current.key = ltree_biggest.key;
                                current.value = ltree_biggest.value;
                                return Ok(ret);
                            }
                            while temp.right.as_ref().unwrap().rpeek().is_some() {
                                temp = temp.right.as_mut().unwrap();
                            }
                            let ltree_biggest = temp.right.take().unwrap();
                            temp.right = ltree_biggest.left;
                            current.key = ltree_biggest.key;
                            current.value = ltree_biggest.value;
                            Ok(ret)
                        }
                    };
                }
                std::cmp::Ordering::Greater => match current.rpeek() {
                    None => return Err(()),
                    Some(x) if x != &key => {
                        current = current.right.as_mut().unwrap();
                        continue;
                    }
                    Some(_) => {
                        return match current.right.as_ref().unwrap().child_state() {
                            0 => Ok(current.right.take().unwrap().value),
                            1 => {
                                let ret = current.right.take().unwrap();
                                current.right = ret.right;
                                Ok(ret.value)
                            }
                            2 => {
                                let ret = current.right.take().unwrap();
                                current.right = ret.left;
                                Ok(ret.value)
                            }
                            _ => {
                                current = current.right.as_mut().unwrap();
                                let ret = current.value.clone();
                                let mut temp: &mut Box<Node<K, V>> = current;
                                while temp.left.as_ref().unwrap().rpeek().is_some() {
                                    temp = temp.left.as_mut().unwrap();
                                }
                                let rtree_smallest = temp.right.take().unwrap();
                                temp.left = rtree_smallest.right;
                                current.key = rtree_smallest.key;
                                current.value = rtree_smallest.value;
                                Ok(ret)
                            }
                        };
                    }
                },
            }
        }
    }

    pub fn breadth_first_traversal(&self) -> String
    where
        K: Display,
    {
        if self.root.is_none() {
            return "**[Empty]**".to_string();
        }
        let mut ret = "".to_string();
        let mut queue = VecDeque::new();
        queue.push_front(self.root.as_ref().unwrap());

        while !queue.is_empty() {
            let node = queue.pop_back().unwrap();
            ret = ret + node.key.to_string().as_str() + ", ";
            match node.child_state() {
                0 => {}
                1 => queue.push_front(node.right.as_ref().unwrap()),
                2 => queue.push_front(node.left.as_ref().unwrap()),
                _ => {
                    queue.push_front(node.left.as_ref().unwrap());
                    queue.push_front(node.right.as_ref().unwrap());
                }
            }
        }

        format!("[{}]", ret.trim_end_matches(", "))
    }

    pub fn pre_order_traversal(&self) -> String
    where
        K: Display,
    {
        fn pre_order<K: Display, V>(node: Option<&Box<Node<K, V>>>) -> String {
            if node.is_none() {
                return "".to_string();
            }

            let node = node.unwrap();
            format!(
                "{}{}{}",
                node.key.to_string() + ", ",
                pre_order(node.left.as_ref()),
                pre_order(node.right.as_ref()),
            )
        }

        if self.root.is_none() {
            return "**[Empty]**".to_string();
        }

        format!("[{}]", pre_order(self.root.as_ref()).trim_end_matches(", "))
    }

    pub fn in_order_traversal(&self) -> String
    where
        K: Display,
    {
        fn in_order<K: Display, V>(node: Option<&Box<Node<K, V>>>) -> String {
            if node.is_none() {
                return "".to_string();
            }

            let node = node.unwrap();
            format!(
                "{}{}{}",
                in_order(node.left.as_ref()),
                node.key.to_string() + ", ",
                in_order(node.right.as_ref()),
            )
        }

        if self.root.is_none() {
            return "**[Empty]**".to_string();
        }

        format!("[{}]", in_order(self.root.as_ref()).trim_end_matches(", "))
    }

    pub fn post_order_traversal(&self) -> String
    where
        K: Display,
    {
        fn post_order<K: Display, V>(node: Option<&Box<Node<K, V>>>) -> String {
            if node.is_none() {
                return "".to_string();
            }

            let node = node.unwrap();
            format!(
                "{}{}{}",
                post_order(node.left.as_ref()),
                post_order(node.right.as_ref()),
                node.key.to_string() + ", ",
            )
        }

        if self.root.is_none() {
            return "**[Empty]**".to_string();
        }

        format!(
            "[{}]",
            post_order(self.root.as_ref()).trim_end_matches(", ")
        )
    }

    pub fn from_slice(slice: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Ord,
    {
        let mut list = Self::new();
        for (k, v) in slice {
            list.insert(k, v);
        }
        list
    }
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    fn foo() {
        let mut bstree = BSTree::new();
        assert!(bstree.remove(0).is_err());
        bstree.insert(0, ());
        assert!(bstree.remove(0).is_ok());
        assert_eq!(bstree.pre_order_traversal(), "**[Empty]**");

        bstree.insert(1, ());
        bstree.insert(2, ());
        bstree.insert(0, ());

        assert!(bstree.remove(1).is_ok());
        assert!(bstree.remove(10).is_err());

        bstree.insert(3, ());
        bstree.insert(5, ());
        bstree.insert(0, ());
        bstree.insert(4, ());

        assert_eq!(bstree.pre_order_traversal(), "[0, 2, 0, 3, 5, 4]");
        assert_eq!(bstree.in_order_traversal(), "[0, 0, 2, 3, 4, 5]");
        assert_eq!(bstree.post_order_traversal(), "[0, 4, 5, 3, 2, 0]");
        assert_eq!(bstree.breadth_first_traversal(), "[0, 2, 0, 3, 5, 4]");

        let mut bstree = BSTree::from_slice([
            ('F', 1u8),
            ('B', 2),
            ('G', 3),
            ('A', 4),
            ('D', 5),
            ('I', 6),
            ('C', 7),
            ('E', 8),
            ('H', 9),
        ]);
        assert_eq!(bstree.pre_order_traversal(), "[F, B, A, D, C, E, G, I, H]");
        assert_eq!(bstree.in_order_traversal(), "[A, B, C, D, E, F, G, H, I]");
        assert_eq!(bstree.post_order_traversal(), "[A, C, E, D, B, H, I, G, F]");
        assert_eq!(
            bstree.breadth_first_traversal(),
            "[F, B, G, A, D, I, C, E, H]"
        );
        assert_eq!(bstree.get('D'), Some(5));
        assert_eq!(bstree.get('F'), Some(1));
        assert_eq!(bstree.remove('Z'), Err(()));
        assert_eq!(bstree.get('C'), Some(7));
        assert_eq!(bstree.remove('F'), Ok(1));
        assert_eq!(bstree.breadth_first_traversal(), "[E, B, G, A, D, I, C, H]");
        assert_eq!(bstree.remove('G'), Ok(3));
        assert_eq!(bstree.breadth_first_traversal(), "[E, B, I, A, D, H, C]");
        assert_eq!(bstree.remove('Z'), Err(()));
    }
}
