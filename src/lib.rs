pub trait Sorter {
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod quick_sort;
pub mod radix_sort;
pub mod selection_sort;

#[cfg(test)]
mod std_sort {
    use super::Sorter;

    struct StdSort;

    impl Sorter for StdSort {
        fn sort<T: Ord>(&self, slice: &mut [T]) {
            slice.sort();
        }
    }

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
