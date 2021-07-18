use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        for i in 0..slice.len() {
            let (xs_smallest, _) = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .expect("slice is non-empty");
            if i != xs_smallest + i {
                slice.swap(i, xs_smallest + i);
            }
        }
    }
}

#[test]
fn works() {
    let mut list = [4, 6, 3, 2, 5, 1];
    SelectionSort.sort(&mut list);
    assert_eq!([1, 2, 3, 4, 5, 6], list);
}

#[test]
fn empty_list() {
    let mut list: [i32; 0] = [];
    let cmp = list.clone();
    SelectionSort.sort(&mut list);
    assert_eq!(cmp, list);
}
