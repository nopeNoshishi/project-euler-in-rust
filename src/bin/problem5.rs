//! Problem5 Smallest multiple
//!
//! 「方針」
//! 最初の数字(1)から最後の数字(任意)まで以下のように評価する
//! - 素数であれば素因数リストに追加する
//! - 素数ではないがある素数の累乗であれば、その素数を素因数リストに追加する。
//! - 上記以外は素因数リストに追加しない
//! - 素因数リストの累積和を算出する
//!

use std::collections::HashSet;

fn main() {
    let test = search_smallest_multiple(20);
    println!("{}", test);
}

fn search_smallest_multiple(end: u64) -> u64 {
    // 1は評価の対象としなくても問題ないため弾く

    // 素因数リストと素数リストを追加する
    let mut prime_factors = Vec::<u64>::new();
    let mut primes = HashSet::<u64>::new();

    for n in 2..end {
        if is_prime(n) {
            prime_factors.push(n);
            primes.insert(n);
        } else {
            match prime_pow(n, primes.clone()) {
                Some(p) => prime_factors.push(p),
                None => continue,
            }
        }
    }

    prime_factors.iter().product()
}

fn is_prime(n: u64) -> bool {
    let lim = (n as f64).sqrt().floor() as u64 + 1;

    for i in 2..lim {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn prime_pow(n: u64, primes: HashSet<u64>) -> Option<u64> {
    // nが自分より小さい素数で何度も割り切れるかを試す
    for p in primes {
        let mut target = n;
        while target % p == 0 {
            target /= p;
        }

        // 累乗になっていれば、最後に残る商は１となる
        if target == 1 {
            return Some(p);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
        assert_eq!(false, is_prime(6));
        assert_eq!(true, is_prime(7));
        assert_eq!(false, is_prime(8));
        assert_eq!(false, is_prime(9));
        assert_eq!(false, is_prime(10));
        assert_eq!(true, is_prime(11));
        assert_eq!(false, is_prime(12));
        assert_eq!(true, is_prime(13));
        assert_eq!(false, is_prime(14));
        assert_eq!(false, is_prime(15));
        assert_eq!(false, is_prime(16));
        assert_eq!(true, is_prime(17));
        assert_eq!(false, is_prime(18));
        assert_eq!(true, is_prime(19));
        assert_eq!(false, is_prime(20));
    }

    #[test]
    fn test_prime_pow() {
        let primes: HashSet<u64> = vec![2].iter().cloned().collect();

        assert_eq!(Some(2), prime_pow(2, primes.clone()));
        assert_eq!(None, prime_pow(3, primes.clone()));
        assert_eq!(Some(2), prime_pow(4, primes.clone()));

        let primes: HashSet<u64> = vec![2, 3, 5, 7].iter().cloned().collect();

        assert_eq!(Some(2), prime_pow(2, primes.clone()));
        assert_eq!(Some(3), prime_pow(3, primes.clone()));
        assert_eq!(Some(2), prime_pow(4, primes.clone()));
        assert_eq!(Some(5), prime_pow(5, primes.clone()));
        assert_eq!(None, prime_pow(6, primes.clone()));
        assert_eq!(Some(7), prime_pow(7, primes.clone()));
        assert_eq!(Some(2), prime_pow(8, primes.clone()));
        assert_eq!(Some(3), prime_pow(9, primes.clone()));
        assert_eq!(None, prime_pow(10, primes.clone()));
        assert_eq!(None, prime_pow(11, primes.clone()));
        assert_eq!(Some(5), prime_pow(25, primes.clone()));
        assert_eq!(Some(3), prime_pow(27, primes.clone()));
        assert_eq!(Some(7), prime_pow(49, primes.clone()));
    }

    #[test]
    fn test_search_smallest_multiple() {
        assert_eq!(2520, search_smallest_multiple(10));
        assert_eq!(232792560, search_smallest_multiple(20));
    }
}
