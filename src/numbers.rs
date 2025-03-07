pub mod prelude {
    pub use super::{factors, prime, primes};
}

pub fn factors(n: u32) -> Vec<u32> {
    let mut ans = Vec::with_capacity(n as usize);
    ans.push(1);
    ans.append(&mut (2..n / 2).filter(|x| x % 2 == 0).collect());
    ans.push(n);
    ans
}

pub fn prime(n: u32) -> bool {
    factors(n).len() == 2
}

pub fn primes(list: impl IntoIterator<Item = u32>) -> impl Iterator<Item = u32> {
    list.into_iter().filter(|&x| prime(x))
}
