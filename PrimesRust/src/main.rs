#![allow(non_snake_case)]
use std::{collections::hash_map::HashMap};
use std::time::{Instant, Duration};
use lazy_static::lazy_static;

const PRIME_COUNT_RESULTS : [(u64, u64); 10] =
        [
            (10, 4),
            (100, 25),
            (1_000, 168),
            (10_000, 1_229),
            (100_000, 9_592),
            (1_000_000, 78_498),
            (10_000_000, 664_579),
            (100_000_000, 5_761_455),
            (1_000_000_000, 50_847_534),
            (10_000_000_000, 455_052_511),
        ];

lazy_static! {
    static ref PRIME_RESULTS_TABLE : HashMap<u64, u64> = PRIME_COUNT_RESULTS.iter().cloned().collect();
}

struct PrimeSieve<const N: usize> {
    sieve_size : u64,
    bits : BitVector<N> 
}

impl<const N: usize> PrimeSieve<N> {
    fn new(sieve_size : u64) -> Self {
        PrimeSieve {
            sieve_size,
            bits : BitVector::<N>::new(true)
        }
    } 

    fn validate_results(&self) -> bool {
        if let Some(value) = PRIME_RESULTS_TABLE.get(&self.sieve_size) {
            return *value == self.count_primes();
        }
        false
    }

    fn count_primes(&self) -> u64 {
        let sum = 
                (0..self.sieve_size)
                .skip(1)
                .step_by(2)
                .fold(0, |acc, x| if self.bits.get_bit(x as usize) { acc + 1 } else { acc } );
                sum
    }

    fn _run_sieve(&mut self) {
        let mut factor : usize = 3;
        let q : usize = f64::sqrt(self.sieve_size as f64) as usize;

        while factor <= q {
            for num in (factor..self.sieve_size as usize).step_by(2) {
                if self.bits.get_bit(num as usize) {
                    factor = num;
                    break;
                }
            }

            for num in (factor * factor..self.sieve_size as usize).step_by(2 * factor as usize) {
                self.bits.clear_bit(num as usize);
            }

            factor += 2;
        }
    }

    fn print_results(&self, show_results : bool, duration : Duration, passes : u32) {
        if show_results {
            print!("2, ");

            for num in (3..self.sieve_size).step_by(2) {
                if self.bits.get_bit(num as usize) {
                    print!("{}, ", num);
                }
            }

            println!("");
        }

        println!("Passes: {}, Time: {}, Avg: {}, Limit: {}, Count: {}, Valid: {}",
                passes,
                duration.as_secs_f32(),
                duration.as_secs_f32() / passes as f32,
                self.sieve_size,
                self.count_primes(),
                self.validate_results()
            );
    }

    // using no iterator increased speed roughly by 1.5x
    fn run_sieve_no_iterators(&mut self) {
        let mut factor : usize = 3;
        let q : usize = f64::sqrt(self.sieve_size as f64) as usize;

        while factor <= q {
            let mut num = factor;
            while num < self.sieve_size as usize {
                if self.bits.get_bit(num) {
                    factor = num;
                    break;
                }
                num += 2;
            }

            let mut num = factor * factor;
            while num < self.sieve_size as usize {
                self.bits.clear_bit(num);
                num += 2 * factor;
            }

            factor += 2;
        }
    }
}


struct BitVector<const N: usize> {
    field : [u64; N]
}

impl<const N: usize> BitVector<N> {
    fn new(set : bool) -> Self {
        if set {
            BitVector {
                field : [0xffff_ffff_ffff_ffff; N]
            }
        }
        else {
            BitVector{
                field : [0; N]
            }
        }
    }

    #[cfg(test)]
    fn set_bit(&mut self ,bit_number : usize) {
        self.field[bit_number >> 6] |= 0x1 << (bit_number & 0x3f);
    }

    fn clear_bit(&mut self ,bit_number : usize) {
        self.field[bit_number >> 6] &= !(0x1 << ((bit_number) & 0x3f));
    }

    fn get_bit(&self ,bit_number : usize) -> bool {
        self.field[bit_number >> 6] & 0x1 << (bit_number & 0x3f) > 0
    }

    #[cfg(test)]
    fn count_set_bits(&self) -> usize {
        let sum = 
        (0..self.field.len() * 64)
        .fold(0, |acc, x| if self.get_bit(x) { acc + 1 } else { acc } );
        sum
    }
}

#[test]
fn test_bit_vector_basic() {
    let mut bit_vector = BitVector::<32>::new(false);
    assert_eq!(0, bit_vector.count_set_bits());
    bit_vector.set_bit(8);
    bit_vector.set_bit(77);
    assert_eq!(2, bit_vector.count_set_bits());
    bit_vector.clear_bit(77);
    assert_eq!(1, bit_vector.count_set_bits());
}

fn main() {
    let mut passes = 0_u32;
    let start = Instant::now();
    const SIEVE_SIZE : usize = 1_000_000; 

    loop {
        let mut prime_sieve = PrimeSieve::<{SIEVE_SIZE / 64 + (SIEVE_SIZE % 64 > 0) as usize * 1}>::new(SIEVE_SIZE as u64);
        prime_sieve.run_sieve_no_iterators();
        passes += 1;

        if start.elapsed().as_micros() >= 5_000_000 {
            prime_sieve.print_results(false, start.elapsed(), passes);
            break;
        }
    }
}
