# Miller-Rabin Primality Test

This repository contains an implementation of the Miller-Rabin primality test in Rust.

## Overview

The Miller-Rabin primality test is a probabilistic algorithm to determine if a given number is likely prime or composite. It is based on the properties of strong pseudoprimes and is widely used in practice due to its efficiency and accuracy.

## Implementation

The `miller_rabin` function in `main.rs` performs the Miller-Rabin primality test. It takes two parameters: `n`, the number to be tested for primality, and `k`, the number of iterations which determine the accuracy of the test. The function returns `true` if `n` is likely prime, and `false` if `n` is composite.

The implementation includes helper functions for decomposing a number into `r` and `d`, and for performing modular exponentiation efficiently.

## Usage

To test a number for primality, modify the `main` function in `main.rs` with the desired values of `n` and `k`, and run the program. The result will be printed to the console.

## Running the Program

To run the program, ensure that Rust and Cargo are installed on your system. Then, navigate to the project directory in the terminal and run the command:

```bash
cargo run
```
