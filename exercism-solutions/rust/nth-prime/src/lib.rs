// pub fn nth(n: u32) -> u32 {
//     let mut primes: Vec<u32> = vec![2];
//     let mut prime_candidate = 3;

//     while primes.len() < (n + 1) as usize {
//         if primes.iter().all(|prime| prime_candidate % *prime != 0) {
//             primes.push(prime_candidate);
//         }

//         prime_candidate += 2;
//     }

//     *primes.last().unwrap()
// }

fn is_prime(number: u32) -> bool {
    !(2..(number as f32).sqrt() as u32 + 1).any(|x| number % x == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}
