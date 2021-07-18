use super::Sorter;

pub struct HeapSort;

impl Sorter for HeapSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {}
}

#[test]
fn works() {
    let mut list = [4, 6, 3, 2, 5, 1];
    HeapSort.sort(&mut list);
    assert_eq!([1, 2, 3, 4, 5, 6], list);
}

#[test]
fn empty_list() {
    let mut list: [i32; 0] = [];
    let cmp = list.clone();
    HeapSort.sort(&mut list);
    assert_eq!(cmp, list);
}
