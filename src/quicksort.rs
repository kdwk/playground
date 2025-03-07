pub mod prelude {
    pub use super::quicksort;
}

pub fn quicksort<T: PartialOrd + Clone>(list: Vec<T>) -> Vec<T> {
    match list[..] {
        [] => list,
        _ => {
            let pivot = list[0].clone();
            let mut smaller = list[1..]
                .into_iter()
                .filter_map(|item| {
                    if item < &pivot {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<T>>();
            let mut bigger = list[1..]
                .into_iter()
                .filter_map(|item| {
                    if item >= &pivot {
                        Some(item.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<T>>();
            (smaller, bigger) = (quicksort(smaller), quicksort(bigger));
            let mut ans = Vec::with_capacity(list.len());
            ans.extend(smaller);
            ans.push(pivot);
            ans.extend(bigger);
            ans
        }
    }
}

pub mod test {
    use std::time::{Duration, Instant};

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
