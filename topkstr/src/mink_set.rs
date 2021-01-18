use std::collections::BinaryHeap;

/// Data structure to maintain k smallest elements in a set.
pub struct MinkSet<T> {
    k: usize,
    heap: BinaryHeap<T>,
}

impl<T: Ord> MinkSet<T> {
    /// Create a [MinkSet] to maintain k smallest elements in it.
    pub fn new(k: usize) -> Self {
        Self {
            k,
            heap: BinaryHeap::default(),
        }
    }

    /// Insert an element with value x in the set.
    pub fn insert(&mut self, x: T) {
        if self.heap.len() < self.k {
            self.heap.push(x);
        } else if let Some(t) = self.heap.peek() {
            if &x < t {
                self.heap.push(x);
            }
        }
        if self.heap.len() > self.k {
            self.heap.pop();
        }
    }

    /// Consumes the [MinkSet] and returns a vector in sorted (ascending) order.
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::MinkSet;

    #[test]
    fn test1() {
        let mut set = MinkSet::new(3);

        for i in 0..100 {
            set.insert(i);
        }
        let vec = set.into_sorted_vec();
        assert_eq!(vec, [0, 1, 2]);
    }
}
