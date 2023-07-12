pub trait Numable {
    fn to_num(self) -> u64;
}

impl Numable for u8 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for i8 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for u16 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for i16 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for u32 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for i32 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for u64 {
    fn to_num(self) -> u64 {
        self
    }
}

impl Numable for i64 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for u128 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

impl Numable for i128 {
    fn to_num(self) -> u64 {
        self as u64
    }
}

// ある桁数以下の素数リストを返す
pub fn digit2primes<N: Numable>(digit: N) -> Vec<u64> {

    let max_num = 10_u64.pow(digit.to_num() as u32);

    let mut candidate = (2..max_num).collect::<Vec<u64>>();
    let mut primes = Vec::new();

    // Limit
    let lim = (max_num as f64).sqrt().floor() as u64;

    // Start
    let mut prime = 2_u64;
    loop {
        primes.push(prime);
        candidate = candidate.drain(..).filter(|x| x % prime != 0).collect();

        // Update next candidate prime
        prime = *candidate.iter().find(|x| **x != 0).unwrap();

        if prime > lim {
            break;
        }
    }

    let rest_prime = candidate
        .iter()
        .filter(|x| **x != 0)
        .collect::<Vec<&u64>>();

    for prime in rest_prime {
        primes.push(*prime);
    }

    primes
}

// ある素数の倍数の数字の候補を落とす
pub fn sieve(prime: u64, mut candidate: Vec<u64>) -> Vec<u64> {
    candidate.drain(..).filter(|x| x % prime != 0).collect()
}
