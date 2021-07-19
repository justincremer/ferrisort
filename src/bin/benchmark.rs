use ferrisort::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::error::Error;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 1000, 10000, 50000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..3 {
            values.shuffle(&mut rand);

            pretty_print("insertion", n, bench(InsertionSort, &values, &counter));
            pretty_print("selection", n, bench(SelectionSort, &values, &counter));
            pretty_print("bubble   ", n, bench(BubbleSort, &values, &counter));
            pretty_print("stable   ", n, bench(StdSort, &values, &counter));
            pretty_print("unstable ", n, bench(StdUnstableSort, &values, &counter));
            pretty_print("quick    ", n, bench(QuickSort, &values, &counter));
            // pretty_print("heap     ", n, bench(HeapSort, &values, &counter));
            // pretty_print("radix    ", n, bench(RadixSort, &values, &counter));
        }
    }
    Ok(())
}

fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}

fn pretty_print(algo: &str, n: usize, res: (usize, f64)) {
    println!("{} {} {} {:.3}s", algo, n, res.0, res.1);
}
