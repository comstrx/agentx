use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::arch::List;

impl List {

    pub fn unique <T: Clone + Eq + Hash> ( items: &[T] ) -> Vec<T> {

        let mut seen = HashSet::new();
        let mut out = Vec::new();

        for item in items {

            if seen.insert(item) { out.push(item.clone()); }

        }

        out

    }

    pub fn all_same <T: PartialEq> ( items: &[T] ) -> bool {

        items.split_first().is_some_and(|( head, tail )| tail.iter().all(|item| item == head))

    }

    pub fn contains <T: PartialEq> ( items: &[T], target: &T ) -> bool {

        items.iter().any(|item| item == target)

    }

    pub fn index_of <T: PartialEq> ( items: &[T], target: &T ) -> Option<usize> {

        items.iter().position(|item| item == target)

    }

    pub fn count <T: PartialEq> ( items: &[T], target: &T ) -> usize {

        items.iter().filter(|item| *item == target).count()

    }

    pub fn first <T> ( items: &[T] ) -> Option<&T> {

        items.first()

    }

    pub fn last <T> ( items: &[T] ) -> Option<&T> {

        items.last()

    }

    pub fn take <T: Clone> ( items: &[T], n: usize ) -> Vec<T> {

        items.iter().take(n).cloned().collect()

    }

    pub fn drop <T: Clone> ( items: &[T], n: usize ) -> Vec<T> {

        items.iter().skip(n).cloned().collect()

    }

    pub fn chunk <T: Clone> ( items: &[T], size: usize ) -> Vec<Vec<T>> {

        if size == 0 { return Vec::new(); }

        items.chunks(size).map(|chunk| chunk.to_vec()).collect()

    }

    pub fn flatten <T: Clone> ( groups: &[Vec<T>] ) -> Vec<T> {

        groups.iter().flatten().cloned().collect()

    }

    pub fn reversed <T: Clone> ( items: &[T] ) -> Vec<T> {

        items.iter().rev().cloned().collect()

    }

    pub fn sorted <T: Ord + Clone> ( items: &[T] ) -> Vec<T> {

        let mut out = items.to_vec();
        out.sort();

        out

    }

    pub fn partition <T: Clone, F: Fn(&T) -> bool> ( items: &[T], pred: F ) -> ( Vec<T>, Vec<T> ) {

        items.iter().cloned().partition(pred)

    }

    pub fn min <T: Ord> ( items: &[T] ) -> Option<&T> {

        items.iter().min()

    }

    pub fn max <T: Ord> ( items: &[T] ) -> Option<&T> {

        items.iter().max()

    }

    pub fn compact <T> ( items: Vec<Option<T>> ) -> Vec<T> {

        items.into_iter().flatten().collect()

    }

    pub fn intersection <T: Clone + Eq + Hash> ( left: &[T], right: &[T] ) -> Vec<T> {

        let other: HashSet<&T> = right.iter().collect();

        let mut seen = HashSet::new();
        let mut out = Vec::new();

        for item in left {

            if other.contains(item) && seen.insert(item) { out.push(item.clone()); }

        }

        out

    }

    pub fn difference <T: Clone + Eq + Hash> ( left: &[T], right: &[T] ) -> Vec<T> {

        let other: HashSet<&T> = right.iter().collect();

        let mut seen = HashSet::new();
        let mut out = Vec::new();

        for item in left {

            if !other.contains(item) && seen.insert(item) { out.push(item.clone()); }

        }

        out

    }

    pub fn union <T: Clone + Eq + Hash> ( left: &[T], right: &[T] ) -> Vec<T> {

        let mut out = left.to_vec();
        out.extend_from_slice(right);

        Self::unique(&out)

    }

    pub fn position <T, F: Fn(&T) -> bool> ( items: &[T], pred: F ) -> Option<usize> {

        items.iter().position(pred)

    }

    pub fn find <T, F: Fn(&T) -> bool> ( items: &[T], pred: F ) -> Option<&T> {

        items.iter().find(|item| pred(item))

    }

    pub fn windows <T: Clone> ( items: &[T], size: usize ) -> Vec<Vec<T>> {

        if size == 0 { return Vec::new(); }

        items.windows(size).map(|window| window.to_vec()).collect()

    }

    pub fn dedup <T: Clone + PartialEq> ( items: &[T] ) -> Vec<T> {

        let mut out: Vec<T> = Vec::with_capacity(items.len());

        for item in items {

            if out.last() != Some(item) { out.push(item.clone()); }

        }

        out

    }

    pub fn counts <T: Clone + Eq + Hash> ( items: &[T] ) -> HashMap<T, usize> {

        let mut out: HashMap<T, usize> = HashMap::new();

        for item in items {

            *out.entry(item.clone()).or_insert(0) += 1;

        }

        out

    }

    pub fn group_by <T: Clone, K: Eq + Hash, F: Fn(&T) -> K> ( items: &[T], key: F ) -> HashMap<K, Vec<T>> {

        let mut out: HashMap<K, Vec<T>> = HashMap::new();

        for item in items {

            out.entry(key(item)).or_default().push(item.clone());

        }

        out

    }

    pub fn zip <A: Clone, B: Clone> ( left: &[A], right: &[B] ) -> Vec<( A, B )> {

        left.iter().cloned().zip(right.iter().cloned()).collect()

    }

    pub fn is_sorted <T: PartialOrd> ( items: &[T] ) -> bool {

        items.is_sorted()

    }

    pub fn max_by_key <T: Clone, K: Ord, F: Fn(&T) -> K> ( items: &[T], key: F ) -> Option<T> {

        items.iter().max_by_key(|item| key(item)).cloned()

    }

    pub fn min_by_key <T: Clone, K: Ord, F: Fn(&T) -> K> ( items: &[T], key: F ) -> Option<T> {

        items.iter().min_by_key(|item| key(item)).cloned()

    }

    pub fn rotate_left <T: Clone> ( items: &[T], n: usize ) -> Vec<T> {

        if items.is_empty() { return Vec::new(); }

        let shift = n % items.len();

        let mut out = items.to_vec();
        out.rotate_left(shift);

        out

    }

    pub fn rotate_right <T: Clone> ( items: &[T], n: usize ) -> Vec<T> {

        if items.is_empty() { return Vec::new(); }

        let shift = n % items.len();

        let mut out = items.to_vec();
        out.rotate_right(shift);

        out

    }

}
