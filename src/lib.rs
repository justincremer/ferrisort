mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;

pub use bubble_sort::BubbleSort;
pub use heap_sort::HeapSort;
pub use insertion_sort::InsertionSort;
pub use quick_sort::QuickSort;
pub use radix_sort::RadixSort;
pub use selection_sort::SelectionSort;

pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

pub struct StdSort;
impl<T: Ord> Sorter<T> for StdSort {
    fn sort(&self, slice: &mut [T]) {
        slice.sort();
    }
}

pub struct StdUnstableSort;
impl<T: Ord> Sorter<T> for StdUnstableSort {
    fn sort(&self, slice: &mut [T]) {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod std_sort {
    use super::{Sorter, StdSort};

    #[test]
    fn works() {
        let mut list = [4, 6, 3, 2, 5, 1];
        StdSort.sort(&mut list);
        assert_eq!([1, 2, 3, 4, 5, 6], list);
    }

    #[test]
    fn empty_list() {
        let mut list: [i32; 0] = [];
        let cmp = list.clone();
        StdSort.sort(&mut list);
        assert_eq!(cmp, list);
    }
}
