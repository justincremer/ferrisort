use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        for mut i in 1..slice.len() {
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn works() {
    let mut list = [4, 6, 3, 2, 5, 1];
    InsertionSort.sort(&mut list);
    assert_eq!([1, 2, 3, 4, 5, 6], list);
}

#[test]
fn empty_list() {
    let mut list: [i32; 0] = [];
    let cmp = list.clone();
    InsertionSort.sort(&mut list);
    assert_eq!(cmp, list);
}
