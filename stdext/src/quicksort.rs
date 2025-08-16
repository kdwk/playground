pub mod prelude {
    pub use super::quicksort;
}

use std::iter::Iterator;

use extend::ext;

pub fn quicksort<T: PartialOrd>(mut list: Vec<T>) -> Vec<T> {
    match list[..] {
        [] => list,
        _ => {
            let pivot = list.pop().unwrap();
            let (smaller, bigger) = list
                .into_iter()
                .map(sorting_flap(|item| item < &pivot))
                .collect2_vec();
            quicksort(smaller)
                .join_with_element(pivot)
                .join_with(quicksort(bigger))
        }
    }
}

#[extend::ext]
impl<T> Vec<T> {
    #[inline]
    fn join_with(mut self, mut other: Vec<T>) -> Vec<T> {
        self.append(&mut other);
        self
    }
    #[inline]
    fn join_with_element(mut self, element: T) -> Vec<T> {
        self.push(element);
        self
    }
}

#[ext]
impl<A, B, I: Iterator<Item = (Option<A>, Option<B>)>> I {
    #[inline]
    fn collect2_vec(mut self) -> (Vec<A>, Vec<B>) {
        let size = self.size_hint().0;
        let mut vec_a = Vec::with_capacity(size);
        let mut vec_b = Vec::with_capacity(size);
        while let Some((opt_a, opt_b)) = self.next() {
            match opt_a {
                Some(a) => vec_a.push(a),
                None => {}
            }
            match opt_b {
                Some(b) => vec_b.push(b),
                None => {}
            }
        }
        (vec_a, vec_b)
    }
}

#[inline]
fn sorting_flap<T>(left_condition: impl Fn(&T) -> bool) -> impl Fn(T) -> (Option<T>, Option<T>) {
    move |item| {
        if left_condition(&item) {
            (Some(item), None)
        } else {
            (None, Some(item))
        }
    }
}

pub mod test {
    use std::time::{Duration, Instant};

    use extend::ext;

    use super::prelude::*;

    pub fn test() {
        let v = vec![9, 3, -1, 0, 3, -56];
        // let (sorted, duration) = benchmark(|| quicksort(v));
        // println!(
        //     "Found ans: {:?} in {}s",
        //     sorted,
        //     duration.as_nanos() as f64 * 1.0 / 1_000_000_000.0
        // )
    }
}
