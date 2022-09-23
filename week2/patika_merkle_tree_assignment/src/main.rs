// Import Crates
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::Digest;
use sha2::Sha256;

// calculate the hashed values from string values.
fn hash_single_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

// main merkle_root function to return expected output.
fn merkle_root(filename: String) -> String {
    // Read Input Data from txt file
    let mut n:u32 = 0;
    let mut vec:Vec<String> = Vec::new();

    // Create vector of strings for leaves
    // Hash inputs and append to vector
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        if index == 0 {n = line.parse().unwrap(); continue};
        vec.push(hash_single_input(&line));
    }

    println!("n: {}", n);

    // Then Write an algorithm that calculates the ROOT
    // Return the root hash as a String
    calculate_merkle_root(vec)
}

// driver merkle root function to calculate algorithm for merkle hash. 
fn calculate_merkle_root(mut vec: Vec::<String>) -> String {
    let mut temp_vec: Vec<String> = Vec::new();
    let mut i = 0;
    let length = vec.len();

    if vec.len() == 1 {
        return vec[0].to_string()
    }  
 
    while i < length {
        if length % 2 != 0 {
            vec.push(vec[length-1].to_owned());
        }
        let mut hash = vec[i].as_str().to_owned() + vec[i+1].as_str();
        hash = hash_single_input(&hash);
        temp_vec.push(hash);
        i += 2;
    }
    vec = temp_vec;
    calculate_merkle_root(vec)
}

fn main() {
    println!("merkle root: {}", merkle_root("input0.txt".to_string()))
}

// tests
// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}