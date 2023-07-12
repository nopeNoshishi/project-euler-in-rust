//! 10001 st Prime
//! https://projecteuler.net/problem=7
//! 
//! 各桁の素数を利用する
//! 1. 対象のn番目の素数が、どの桁数にあるかを特定する。　
//! 2. その桁の全ての数字を素数候補リストとする。
//! 3. その桁数以内の素数を全て取り出し、2で作成したリストをその素数の篩にかける。
//! 4. 桁数ごとの素数の数をうまく利用して、残った素数リストから対象のn番目の素数を取り出す。

use rust_euler::*;

fn main() {

    let nth_prime = search_nth_prime(1432);

    println!("Answer: {}", nth_prime);
}

fn search_nth_prime<N: Numable>(nth: N) -> u64 {
    let nth = nth.to_num();

    let mut prime_digit = 1;
    loop {
        if prime_bank(prime_digit) > nth as u64 {
            break; 
        }

        prime_digit += 1
    }

    let target_digit = prime_digit - 1;
    let base = prime_bank(target_digit);

    let primes = digit2primes(target_digit);
    let mut candidate = (10_u64.pow(target_digit as u32)..10_u64.pow(target_digit as u32 + 1)).collect::<Vec<u64>>();

    // 元の数字の桁数より１つ小さい桁の素数全体で篩をかけると残りは素数になる。
    for p in primes {
        candidate = sieve(p, candidate)
    }

    candidate[(nth - base - 1) as usize]
}

fn prime_bank(digit: u64) -> u64 {

    match digit {
        1 => 4,
        2 => 25,
        3 => 168,
        4 => 1229,
        5 => 9592,
        6 => 78498,
        7 => 664579,
        8 => 5761455,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_nth_prime() {
        assert_eq!(13, search_nth_prime(6));
        assert_eq!(541, search_nth_prime(100));
        assert_eq!(11941, search_nth_prime(1432));
    }

    #[test]
    fn test_prime_bank() {
        assert_eq!(78498, prime_bank(6));
        assert_eq!(0, prime_bank(12));
    }
}
