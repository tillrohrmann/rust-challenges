use euler_64::*;

fn main() {
    let primes = Primes::new(10000);

    let num = (1..=10000).map(|i| calculate_sequence_length(i, &primes)).filter(|i| i % 2 == 1).count();

    println!("{}", num);
}
