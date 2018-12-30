fn main() {
    let primes = calculate_primes(100000);

    println!("Primes {:?}", primes);
}

fn calculate_primes(number_primes: u32) -> Vec<u32> {
    let mut primes = vec![0; number_primes as usize];
    let mut current_number = 2;

    for i in 0..primes.len() {
        while !is_prime(current_number) {
            current_number += 1;
        }

        primes[i] = current_number;
        current_number += 1;
    }

    primes
}

fn is_prime(number: u32) -> bool {
    let sqrt = (number as f64).sqrt() as u32 + 1;

    for i in 2..sqrt {
        if number % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

}
