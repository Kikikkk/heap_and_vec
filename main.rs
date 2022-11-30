use std::collections::BinaryHeap;

fn mins<TT: PartialOrd + std::cmp::Ord + Clone, T: Iterator<Item = TT>>(mut it: T, n: u8) -> Vec<TT>{
    let mut heap = BinaryHeap::new();
    for i in 0..n {
          heap.push(it.next().unwrap().clone());
    }
    let vec = heap.into_sorted_vec();
    return vec;
}

fn main() {
    let mut vec = vec![1,45,53,53,4,45,641,5534];
    vec.sort();
    let mins = mins(vec.iter(), 7);
    for v in mins{
        print!("{} ", v);
    }
}
