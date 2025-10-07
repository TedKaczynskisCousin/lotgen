use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

fn main() {
    // File must exist in the current path
    if let Ok(lines) = read_lines("src/LotteryUSA.txt") {
        // Consumes the iterator, returns an (Optional) String
        let regex = Regex::new(r"[0-9]+").unwrap();
        
        // Vector of vectors with lottery numbers. This is mainly for show and isn't used in any calculations. Maybe in the future, inter-set correlations can be made with this information
        let mut winning_vectors = Vec::new();
        // A non-nested vector that will hold every occurrence of a winning number. Used in calculation)
        let mut winning_nums = Vec::new();
        
        // We are using a counter to go through the file line-by-line and extract the lottery numbers
        let mut counter = 0;
        for line in lines.flatten() {
            counter += 1;
            // We're using a modulo operation to select the second line out of each group of five, since that's where the lottery numbers are
            if (counter % 5) == 2 {
                // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
                // Converts would be to vector of strings to vector of ints within same line!
                let mut lotto_nums: Vec<i32> = regex.find_iter(&line).map(|m| m.as_str()).map(|s| s.parse().unwrap()).collect();
                
                // Push the extracted lottery numbers into the respective variable
                winning_vectors.push(lotto_nums.clone());
                winning_nums.append(&mut lotto_nums)
            }
        }
        //println!("{:#?}", winning_vectors);
        //println!("{:?}", winning_nums);
        
        
        // Setup random number gen
        let lower_bound = winning_nums.iter().min().unwrap();
        let upper_bound = winning_nums.iter().max().unwrap();
        
        // Create an array to hold weights for each possible number, later used to influence random number picking. Array size is just a random big number
        let mut original_weights = [0; 99];
        
        for num in *lower_bound..*upper_bound+1 {
            // Generate weights based on count of number / total amount of numbers
            let num_count = winning_nums.iter().filter(|x| **x == num).count();
            let mut num_weight: f32 = num_count as f32 / winning_nums.len() as f32;
            // Invert the weight via reciprocal method (since WeightedIndex does need them to sum to one)
            num_weight = 1.0 / num_weight;
            
            // Final weight value must be multiplied by 10000 in order to avoid being truncated to zero and retain detail
            original_weights[(num - 1) as usize] = (num_weight * 10000.0) as usize;
            
            println!("number: {:?}", num);
            println!("appeared: {:?} times", num_count);
            println!("weight: {:?}\n", num_weight);
        }
        //println!("{:?}\n", original_weights);
        
        
        // Setup RNG stuff
        let mut rng = thread_rng();
        
        // Generate the lucky numbers (10 times)!
        for _i in 0..10 {
            // Generate weighted distribution
            let mut dist = WeightedIndex::new(&original_weights).unwrap();
            
            let mut generated_nums = [0, 0, 0, 0, 0];
            for n in 0..5 {
                // Select a random number and save it to the final five
                let sampled_index = dist.sample(&mut rng);
                let random_number = winning_nums[sampled_index];
                generated_nums[n as usize] = random_number;
                
                // Set picked number weight to zero to prevent duplicate picking
                dist.update_weights(&[(sampled_index, &0)]).unwrap();
            }
            generated_nums.sort();
            
            // Display winning numbers
            println!("{:?}", generated_nums);
        }
        
        
    // If file not found
    } else {
        println!("\nLottery Numbers file was not found!
            \nPlease visit LotteryUSA and export numbers into this directory after removing the first 2 lines of the TXT file.
            \nThe filename should be 'LotteryUSA.txt' and is case sensitive.
        ")
    }
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
