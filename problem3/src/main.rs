//! Problem3 Largest prime factor
//!
//! 「方針」
//! 与えられた数字以下の素数をアリストテレスの篩で探索しながら、
//! 素因数分解を行い、分解し終えた時点で処理を終える。
//!

fn main() {
    let base = 600851475143_u64;
    println!("{:?}", search_factor(base));
}

pub fn search_factor(mut base: u64) -> Vec<u64> {
    // 素因数の上限
    let lim = (base as f64).sqrt().floor() as u64;

    // 素数の可能性のある集合
    let mut candidate = (2..lim + 1).collect::<Vec<u64>>();

    // 最初の素数
    let mut prime = 2;

    // 素因数（重複あり）
    let mut prime_factors = Vec::new();

    loop {
        // 与えられた数を素数で割ってみる
        while let 0 = base % prime {
            prime_factors.push(prime);
            base /= prime;
        }

        // 全ての素因数がわかった段階で作業を切り上げる
        if base == 1 {
            break;
        }

        // まだ素数が見つかってない場合、篩にかけて次に大きな素数を割り当てる
        sieve(prime, &mut candidate);
        prime = match (*candidate).iter().find(|x| **x != 0) {
            Some(p) => *p,
            None => break,
        };
    }

    prime_factors.dedup();

    prime_factors
}

// 篩にかけて引数で割れる数は全て0に置き換える。
pub fn sieve(prime: u64, candidate: &mut [u64]) {
    for slot in candidate.iter_mut() {
        if *slot % prime == 0 {
            *slot = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let mut candidate: Vec<u64> = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];

        sieve(2, &mut candidate);

        assert_eq!(candidate, vec![0, 3, 0, 5, 0, 7, 0, 9, 0])
    }

    #[test]
    fn test_search() {
        let base = 13195;

        assert_eq!(search_factor(base), vec![5, 7, 13, 29])
    }
}
