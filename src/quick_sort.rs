use super::Sorter;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        match slice.len() {
            0 | 1 => {}
            2 => {
                if slice[0] > slice[1] {
                    slice.swap(0, 1);
                }
            }
            _ => {
                let (pivot, xs) = slice.split_first_mut().expect("slice is non-empty");
                let mut left = 0;
                let mut right = xs.len() - 1;
                while left <= right {
                    if &xs[left] <= pivot {
                        left += 1;
                    } else if &xs[right] > pivot {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    } else {
                        xs.swap(left, right);
                        left += 1;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                }

                slice.swap(0, left);
                let (left, right) = slice.split_at_mut(left);
                assert!(left.last() <= right.first());
                self.sort(left);
                self.sort(&mut right[1..]);
            }
        }
    }
}

#[test]
fn works() {
    let mut list = [4, 6, 3, 2, 5, 1];
    QuickSort.sort(&mut list);
    assert_eq!([1, 2, 3, 4, 5, 6], list);
}

#[test]
fn empty_list() {
    let mut list: [i32; 0] = [];
    let cmp = list.clone();
    QuickSort.sort(&mut list);
    assert_eq!(cmp, list);
}
