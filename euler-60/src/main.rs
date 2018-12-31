use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct PrimeNumbers {
    ascending: Vec<usize>,
    set: HashMap<usize, usize>,
}

impl PrimeNumbers {
    fn new(numbers: usize) -> PrimeNumbers {
        let mut sieve = vec![true; numbers];

        sieve[0] = false;
        sieve[1] = false;

        for i in 2..sieve.len() {
            if sieve[i] {
                for j in ((2 * i)..sieve.len()).step_by(i) {
                    sieve[j] = false;
                }
            }
        }

        let mut counter = 0;

        for i in 0..sieve.len() {
            if sieve[i] {
                counter += 1;
            }
        }

        let mut ascending = vec![0; counter];
        let mut set = HashMap::with_capacity(counter);

        counter = 0;

        for i in 0..sieve.len() {
            if sieve[i] {
                ascending[counter] = i;
                set.insert(i, counter);

                counter += 1
            }
        }

        PrimeNumbers {
            ascending,
            set
        }
    }

    fn is_prime(&self, prime_candidate: usize) -> Result<bool, ()> {

        if prime_candidate <= self.largest_prime() {
            Ok(self.set.contains_key(&prime_candidate))
        } else {
            Err(())
        }
    }

    fn largest_prime(&self) -> usize {
        self.ascending[self.ascending.len() - 1]
    }

    fn get(&self, idx: usize) -> usize {
        self.ascending[idx]
    }
}

struct Solution<'a> {
    prime_numbers: &'a PrimeNumbers,
    prime_pairs: usize
}

#[derive(Debug, PartialEq, Clone)]
struct SolutionCandidate {
    indices: Vec<usize>,
    primes: Vec<usize>
}

impl SolutionCandidate {
    fn new(indices: Vec<usize>, primes: Vec<usize>) -> SolutionCandidate {
        SolutionCandidate {
            indices,
            primes
        }
    }

    fn len(&self) -> usize {
        self.indices.len()
    }

    fn generate_primes(&self, prime_numbers: &PrimeNumbers) -> Vec<usize> {
        self.indices.iter().map(|&idx| prime_numbers.ascending[idx]).collect()
    }
}

impl PartialOrd<SolutionCandidate> for SolutionCandidate {
    fn partial_cmp(&self, other: &SolutionCandidate) -> Option<Ordering> {
        let self_sum: usize = self.primes.iter().sum();
        let other_sum: usize = other.primes.iter().sum();

        other_sum.partial_cmp(&self_sum)
    }
}

impl Eq for SolutionCandidate {}

impl Ord for SolutionCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Solution<'a> {
    fn new(prime_numbers: &'a PrimeNumbers, prime_pairs: usize) -> Solution<'a> {
        Solution{
            prime_numbers,
            prime_pairs
        }
    }

    fn find_prime_pairs(&self) -> Option<Vec<usize>> {
        println!("Calculate solution");
        let mut heap = BinaryHeap::new();

        heap.push(SolutionCandidate::new(vec![1], vec![self.prime_numbers.get(1)]));

        let mut counter = 0;

        while *heap.peek().map(|c: &SolutionCandidate| c.len() != self.prime_pairs).get_or_insert(false) {
            let solution_candidate = heap.pop().unwrap();

            if counter % 100 == 0 {
                println!("{:?}", solution_candidate);
            }

            counter += 1;

            let new_pair_solution_candidate = self.new_pair_solution_candidate(&solution_candidate);
            let next_solution_candidate = self.next_solution_candidate(solution_candidate);

            next_solution_candidate.map(|c| heap.push(c));
            new_pair_solution_candidate.map(|c| heap.push(c));
        }

        heap.pop().map(|c| c.generate_primes(self.prime_numbers))
    }

    fn new_pair_solution_candidate(&self, solution_candidate: &SolutionCandidate) -> Option<SolutionCandidate> {
        let last_index = solution_candidate.indices.len() - 1;

        let new_candidate = self.find_next_candidate(
            &solution_candidate.indices[..],
            solution_candidate.indices[last_index] + 1);

        match new_candidate {
            Ok(candidate) => {
                let mut new_indices = solution_candidate.indices.clone();
                let mut new_primes = solution_candidate.primes.clone();
                new_indices.push(candidate);
                new_primes.push(self.prime_numbers.get(candidate));

                Some(SolutionCandidate {
                    indices: new_indices,
                    primes: new_primes,
                })
            }
            Err(_) => None
        }
    }

    fn next_solution_candidate(&self, mut solution_candidate: SolutionCandidate) -> Option<SolutionCandidate> {
        let last_index = solution_candidate.indices.len() - 1;
        let new_candidate = self.find_next_candidate(
            &solution_candidate.indices[0..last_index],
            solution_candidate.indices[last_index] + 1);

        match new_candidate {
            Ok(candidate) => {
                solution_candidate.indices[last_index] = candidate;
                solution_candidate.primes[last_index] = self.prime_numbers.get(candidate);

                Some(solution_candidate)
            }
            Err(_) => None
        }


    }

    fn find_next_candidate(&self, others: &[usize], start: usize) -> Result<usize, ()> {
        let mut new_candidate = start;

        while !self.is_new_pair(others, new_candidate)? {
            new_candidate += 1
        }

        Ok(new_candidate)
    }

    fn is_new_pair(&self, others: &[usize], new_candidate: usize) -> Result<bool, ()> {
        for &other in others {
            if !self.is_valid_prime_pair(self.prime_numbers.get(other), self.prime_numbers.get(new_candidate))? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn validate_prime_pairs(&self, pairs: &[usize]) -> Result<bool, ()> {
        for i in 0..(pairs.len() - 1) {
            for j in (i + 1)..pairs.len() {
                if !self.is_valid_prime_pair(self.prime_numbers.ascending[pairs[i]], self.prime_numbers.ascending[pairs[j]])? {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn is_valid_prime_pair(&self, a: usize, b: usize) -> Result<bool, ()> {
        if a == 5 || b == 5 {
            return Err(());
        }

        let x = self.prime_numbers.is_prime(common::concatenate(a, b))?;

        if x {
            Ok(self.prime_numbers.is_prime(common::concatenate(b, a))?)
        } else {
            Ok(false)
        }
    }
}

mod common {
    const DIGITS:[usize; 11] = [10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 10000000000, 100000000000];

    pub fn is_prime(number: usize) -> bool {
        let sqrt = (number as f64).sqrt() as usize + 1;

        for i in 2..sqrt {
            if number % i == 0 {
                return false;
            }
        }

        true
    }

    pub fn concatenate(a: usize, b: usize) -> usize {
        let factor = calculate_factor(b);

        a * factor + b
    }

    fn calculate_factor(number: usize) -> usize {
        for i in 0..DIGITS.len() {
            if number < DIGITS[i] {
                return DIGITS[i];
            }
        }

        panic!("Could not find factor for number {}.", number);
    }
}

fn main() {
    let primes = PrimeNumbers::new(100000000);
    let solution = Solution::new(&primes, 5);
    let prime_pairs = solution.find_prime_pairs().unwrap();

    println!("Prime pairs {:?}", prime_pairs);
    let sum: usize = prime_pairs.iter().sum();
    println!("Sum of prime pairs {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Index;

    #[test]
    fn test_wrong_prime_pairs() {
        let primes = PrimeNumbers::new(10000);
        let solution = Solution::new(&primes, 3);
        assert!(!solution.validate_prime_pairs(&vec![0, 1, 4]).unwrap());
    }

    #[test]
    fn test_prime_pairs() {
        let primes = PrimeNumbers::new(700000);
        let solution = Solution::new(&primes, 4);
        assert!(solution.validate_prime_pairs(&vec![*primes.set.index(&3), *primes.set.index(&7), *primes.set.index(&109), *primes.set.index(&673)]).unwrap());
    }

    #[test]
    fn test_wrong_prime_pair() {
        let primes = PrimeNumbers::new(10000);
        let solution = Solution::new(&primes, 2);
        assert!(!solution.is_valid_prime_pair(2, 11).unwrap());
    }

    #[test]
    fn test_number_concatenate() {
        let a = 111;
        let b = 222;

        assert_eq!(common::concatenate(a, b), 111222);
        assert_eq!(common::concatenate(b, a), 222111);
    }

    #[test]
    fn test_heap() {
        let mut heap = BinaryHeap::new();
        let first = SolutionCandidate::new(vec![2, 3], vec![5, 7]);
        let second = SolutionCandidate::new(vec![1, 2], vec![3, 5]);
        let third = SolutionCandidate::new(vec![2], vec![5]);

        heap.push(first.clone());
        heap.push(second.clone());
        heap.push(third.clone());

        let first_pop = heap.pop().unwrap();
        let second_pop = heap.pop().unwrap();
        let third_pop = heap.pop().unwrap();

        assert_eq!(first_pop, third);
        assert_eq!(second_pop, second);
        assert_eq!(third_pop, first);
    }
}
