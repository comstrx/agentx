use std::collections::HashSet;
use std::hash::Hash;

use super::arch::List;

impl List {

    /// Unique items, preserving first-seen order.
    pub fn unique<T: Clone + Eq + Hash> ( items: &[T] ) -> Vec<T> {

        let mut seen = HashSet::new();
        let mut out = Vec::new();

        for item in items {

            if seen.insert(item) {
                out.push(item.clone());
            }
        }

        out

    }

    /// True when the slice is non-empty and every item is equal.
    pub fn all_same<T: PartialEq> ( items: &[T] ) -> bool {

        items.split_first().is_some_and(|(head, tail)| tail.iter().all(|item| item == head))

    }

}
