// Import the rand crate to generate random numbers
use rand::Rng;
use std::io;

   

// Function to perform the Miller-Rabin primality test
// Parameters:
// - n: the number to be tested for primality
// - k: the number of iterations, which determines the accuracy of the test
// Returns:
// - true if n is likely prime, false if n is composite
fn miller_rabin(n: u64, k: u64) -> bool {
    // Base cases for n <= 3
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    // If n is even, it's not prime
    if n % 2 == 0 {
        return false;
    }

    // Decompose n into r and d, where n-1 = (2^r) * d
    let (r, d) = decompose_n(n);

    // Repeat the primality test k times
    'witness_loop: for _ in 0..k {
        // Generate a random witness 'a' in the range [2, n-2]
        let a = rand::thread_rng().gen_range(2..n - 2);
        // Calculate x = (a^d) mod n
        let mut x = modular_exp(a, d, n);
        // Check for the simplest cases
        if x == 1 || x == n - 1 {
            continue;
        }
        // Repeat r-1 times
        for _ in 0..r - 1 {
            // Calculate x = (x^2) mod n
            x = modular_exp(x, 2, n);
            // If x is n-1, move to the next iteration
            if x == n - 1 {
                continue 'witness_loop;
            }
        }
        // If none of the iterations indicate primality, return false
        return false;
    }

    // If all iterations indicate primality, return true
    true
}

// Function to decompose n into r and d, where n-1 = (2^r) * d
// Parameter:
// - n: the number to be decomposed
// Returns:
// - a tuple containing r and d
fn decompose_n(n: u64) -> (u64, u64) {
    let mut d = n - 1;
    let mut r = 0;
    // Divide d by 2 until it's odd, counting the number of divisions in r
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    (r, d)
}

// Function to perform modular exponentiation (base^exp mod modulus) efficiently
// Parameters:
// - base: the base of the exponentiation
// - exp: the exponent
// - modulus: the modulus
// Returns:
// - the result of the modular exponentiation
fn modular_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    // Perform exponentiation using bitwise operations
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

// Main function to test the Miller-Rabin primality test
fn main() {
    // let n = 17977; 
    let k = 2; 

    let mut number = String::new();
    println!("Enter a value to check if its a prime number or not");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let number: u64 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Please enter a valid unsigned integer.");
            return;
        }
    };



    // Perform the test and print the result
    if miller_rabin(number, k) {
        println!("{} is likely prime", number);
    } else {
        println!("{} is composite", number);
    }
}