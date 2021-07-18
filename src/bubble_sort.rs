use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        if slice.len() > 1 {
            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in 0..(slice.len() - 1) {
                    if slice[i] > slice[i + 1] {
                        slice.swap(i, i + 1);
                        swapped = true;
                    }
                }
            }
        }
    }
}

#[test]
fn works() {
    let mut list = [4, 6, 3, 2, 5, 1];
    BubbleSort.sort(&mut list);
    assert_eq!([1, 2, 3, 4, 5, 6], list);
}

#[test]
fn empty_list() {
    let mut list: [i32; 0] = [];
    let cmp = list.clone();
    BubbleSort.sort(&mut list);
    assert_eq!(cmp, list);
}
