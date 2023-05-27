//! Problem1 Multiples of 3 or 5
//!
//! 「方針」
//! 集合の考えで解いていく。互いの倍数の集合の合計を足して、
//! 最小公倍数の集合の合計をひく。
//!

use num::integer::lcm;

fn main() {
    let sum = sum_multiples(1000, 3, 5);
    println!("Sum: {}", sum);
}

fn sum_multiples(base: u64, p: u64, q: u64) -> u64 {
    // 最小公倍数
    let lcm = lcm(p, q);

    // 各集合の合計
    let sum_set_p = (0..base).filter(|x| x % p == 0).sum::<u64>();
    let sum_set_q = (0..base).filter(|x| x % q == 0).sum::<u64>();
    let sum_set_lcm = (0..base).filter(|x| x % lcm == 0).sum::<u64>();

    sum_set_p + sum_set_q - sum_set_lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_multiples(10, 3, 5), 23);
        assert_eq!(sum_multiples(1000, 3, 5), 233168);
    }
}
