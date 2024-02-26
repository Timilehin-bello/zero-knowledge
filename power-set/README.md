# Power Set Generator

This Rust program defines a function called `power_set` that generates the power set of a given set of characters. It also includes a `main` function to demonstrate the usage of the `power_set` function.

## Function: power_set

### Description

The `power_set` function takes a reference to a slice of characters and returns a vector of vectors of characters representing the power set of the input set.

### Parameters

- `items`: A reference to a slice of characters representing the input set for which the power set needs to be generated.

### Return Value

A vector of vectors of characters representing the power set of the input set.

### Algorithm

1. Get the length of the input slice.
2. Initialize an empty vector called `result` to store the power set.
3. Iterate over all possible subsets of the input set.
   - For each subset, iterate over each element of the input set and add the corresponding elements to the subset based on the binary representation of the subset index.
4. Add each subset to the result vector.
5. Return the power set.

## Function: main

### Description

The `main` function demonstrates the usage of the `power_set` function by generating the power set of the input set containing the characters 'a', 'b', and 'c'.

### Usage

To run the program, simply execute the Rust file containing the code. Upon execution, the program will generate and print the power set of the input set.

## Example

```rust
fn main() {
    let input_set: Vec<char> = vec!['a', 'b', 'c'];
    let result = power_set(&input_set);
    println!("{:?}", result);
}

This will output the power set of the input set:

[[], ['a'], ['b'], ['a', 'b'], ['c'], ['a', 'c'], ['b', 'c'], ['a', 'b', 'c']]
```
