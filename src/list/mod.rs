pub mod linked_list;

pub mod doubly_linked_list;

trait List<T>: Sized {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn push_front(&mut self, value: T);
    fn pop_front(&mut self) -> Option<T>;
    fn push_back(&mut self, value: T);
    fn pop_back(&mut self) -> Option<T>;
    fn insert(&mut self, idx: usize, value: T) -> Result<(), &str>;
    fn remove(&mut self, idx: usize) -> Option<T>;
    fn from_slice(slice: impl IntoIterator<Item = T>) -> Self {
        let mut list = Self::new();
        for i in slice {
            list.push_back(i);
        }
        list
    }
}
