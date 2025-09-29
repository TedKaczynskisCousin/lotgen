//comment?: read file -> do thing -> spit out numbers -> user input

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::Rng;

fn main() {
    
    // File must exist in the current path
    if let Ok(lines) = read_lines("src/LotteryUSA.txt") {
        // Consumes the iterator, returns an (Optional) String
        let regex = Regex::new(r"[0-9]+").unwrap();
        // Vector of vectors with lottery numbers
        let mut lottoVector = Vec::new();
        
        // Format lottery number file and stuff
        let mut counter = 0;
        for line in lines.flatten() {
            counter += 1;
            
            if (counter % 2) == 0 {
                // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
                // Converts would be to vector of strings to vector of ints within same line!
		let lotto_nums: Vec<i32> = regex.find_iter(&line).map(|m| m.as_str()).map(|s| s.parse().unwrap()).collect();
		lottoVector.push(lotto_nums);
            }
        }
        //println!("{:#?}", lottoVector);
        
        
        // Create new vector that doesnt have nesting (used in calculation)
        let mut lotto_vector_simple = Vec::new();
        let lotvec_size = lottoVector.iter().count();
        for n in 0..lotvec_size {
            lotto_vector_simple.push(lottoVector[n][0]);
            lotto_vector_simple.push(lottoVector[n][1]);
            lotto_vector_simple.push(lottoVector[n][2]);
            lotto_vector_simple.push(lottoVector[n][3]);
            lotto_vector_simple.push(lottoVector[n][4]);
        }
        //println!("{:?}", lotto_vector_simple);
        
        
        // Setup random number gen
        let choices = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
            41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 59, 60
        ];
        
        let mut weights = [0; 60];
        for num in 1..61 {
            // Generate weights based on count of number / total amount of numbers * 1000 (to get rid of decimals)
            let num_count = lotto_vector_simple.iter().filter(|x| **x == num).count();
            let total = lotto_vector_simple.iter().count();
            let num_weight = (num_count * 1000)/total;
            weights[(num - 1) as usize] = num_weight;
            
            println!("number: {:?}", num);
            println!("appeared: {:?} times", num_count);
            println!("weight: {:?}\n", num_weight);
        }
        //println!("{:?}\n", weights);
        
        
        // Generate the lucky numbers!
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        
        let mut generated_nums = [0, 0, 0, 0, 0];
        for n in 0..5 {
            let mut random_number = choices[dist.sample(&mut rng)];
            generated_nums[n as usize] = random_number;
            weights[(random_number - 1) as usize] = 0;
        }
        generated_nums.sort();
        
        // Generate powerball
        let powerball = rand::thread_rng().gen_range(1..5);
        // Display winning numbers
        println!("{:?}, {}", generated_nums, powerball);
        
        
    // If file not found
    } else {
        println!("\nLottery Numbers file was not found!
            \nPlease visit LotteryUSA and export numbers into this directory.
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
