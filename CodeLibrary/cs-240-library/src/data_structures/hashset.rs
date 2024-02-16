///////////////////////////////////////////////////////////////////////////////

use std::fmt;

use super::linked_list::single_linked_list::solution::LinkedList;

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, PartialOrd, Debug)]
pub struct HashSet<T>
where
    T: Ord + fmt::Debug,
    Vec<u8>: From<T>,
{
    inner: Vec<LinkedList<T>>,
    size: usize,
    radix: usize,
}

//---------------------------------------------------------------------------//

impl<T> HashSet<T>
where
    T: Ord + fmt::Debug + Copy,
    Vec<u8>: From<T>,
{
    pub fn new() -> Self {
        Self {
            inner: vec![],
            size: 0,
            radix: 29,
        }
    }

    //.......................................................................//

    pub fn with_radix(mut self, radix: usize) -> Self {
        self.radix = radix;

        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.inner = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            self.inner.push(LinkedList::new());
        }

        self
    }

    //.......................................................................//

    fn hash(&self, val: T) -> usize {
        let mut res = 0;

        for token in Vec::<u8>::from(val) {
            res = res * self.radix + usize::from(token);
        }

        res % self.inner.len()
    }

    //.......................................................................//

    pub fn add(&mut self, val: T) {
        let hash = self.hash(val);

        if let Some(_) = self.inner[hash].search(&val) {
        } else {
            self.inner[hash].push(val);
            self.size += 1;
        }
    }

    //.......................................................................//

    pub fn remove(&mut self, val: T) {
        let hash = self.hash(val);

        if let Some(index) = self.inner[hash].search(&val) {
            self.inner[hash].delete(index);
            self.size -= 1;
        }
    }

    //.......................................................................//

    pub fn contains(&self, val: T) -> bool {
        let hash = self.hash(val);

        self.inner[hash].search(&val).is_some()
    }

    //.......................................................................//

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    //.......................................................................//

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.inner.len()
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let mut table = HashSet::new().with_capacity(500);

        assert!(table.is_empty());

        table.add("a");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));
        assert_eq!(table.size(), 1);

        table.remove("a");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert_eq!(table.size(), 0);

        table.add("a");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert_eq!(table.size(), 1);

        table.add("b");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(table.contains("b"));
        assert_eq!(table.size(), 2);

        table.add("c");

        assert!(!table.is_empty());
        assert!(table.contains("a"));
        assert!(table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.size(), 3);

        table.remove("a");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.size(), 2);

        table.remove("b");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.size(), 1);

        table.add("c");

        assert!(!table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(table.contains("c"));
        assert_eq!(table.size(), 1);

        table.remove("c");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));

        table.remove("c");

        assert!(table.is_empty());
        assert!(!table.contains("a"));
        assert!(!table.contains("b"));
        assert!(!table.contains("c"));
    }
}

///////////////////////////////////////////////////////////////////////////////
