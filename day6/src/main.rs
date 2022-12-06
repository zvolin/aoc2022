use std::collections::{hash_map::RandomState, HashSet};
use std::hash::Hash;

static INPUT: &[u8] = include_bytes!("../input");

trait FromSlice<T> {
    fn from_slice(slice: &[T]) -> Self;
}

impl<T: Eq + Hash + Copy> FromSlice<T> for HashSet<T, RandomState> {
    fn from_slice(slice: &[T]) -> Self {
        Self::from_iter(slice.iter().copied())
    }
}

fn find_marker<T: Eq + Hash + Copy>(marker_len: usize, input: &[T]) -> usize {
    input
        .windows(marker_len)
        .take_while(|&w| HashSet::from_slice(w).len() != marker_len)
        .count()
        + marker_len
}

fn main() {
    // part 1
    println!("{}", find_marker(4, INPUT));

    // part 2
    println!("{}", find_marker(14, INPUT));
}
