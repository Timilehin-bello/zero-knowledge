// Define a function called power_set that takes a reference to a slice of characters and returns a vector of vectors of characters
fn power_set(items: &[char]) -> Vec<Vec<char>> {
    // Get the length of the input slice
    let n = items.len();
    // Initialize an empty vector called result to store the power set
    let mut result = Vec::new();
    // Iterate over all possible subsets of the input set
    for i in 0..(1 << n) {
        // Initialize an empty vector called subset to store the current subset
        let mut subset = Vec::new();
        // Iterate over each element of the input set
        for j in 0..n {
            // Check if the j-th bit of the binary representation of i is set
            if (i >> j) & 1 == 1 {
                // If the bit is set, add the corresponding element to the subset
                subset.push(items[j]);
            }
        }
        // Add the current subset to the result vector
        result.push(subset);
    }
    // Return the power set
    result
}

// Define the main function
fn main() {
    // Create a vector called input_set containing the characters 'a', 'b', and 'c'
    let input_set: Vec<char> = vec!['a', 'b', 'c'];
    // Call the power_set function with a reference to the input_set and store the result
    let result = power_set(&input_set);
    // Print the result
    println!("{:?}", result);
}