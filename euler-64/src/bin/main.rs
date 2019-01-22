use euler_64::*;

fn main() {
    let sqrt = sqrt(23);

    println!("{:?}", sqrt);

    let fraction = fraction(15, 10);

    let primes = Primes::new(1000);

    println!("{:?}", cancel(&primes,fraction));
}
