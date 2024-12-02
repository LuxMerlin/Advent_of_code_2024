use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Day 1 Part 1");
    // open and parse file
    let mut file = File::open("src/input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("result: {}", get_part1_anwser(&contents));
}

// parse the input line by line
// for each line add them to 2 vectors
// sort by smallest to largest
// calculate the difference between the 2 vectors line by line and add to a count that is the sum of all the differences
fn get_part1_anwser(contents: &str) -> u32{
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in contents.lines() {
       let data: Vec<String> = line.split_whitespace().map(str::to_string).collect();
       left.push(data[0].parse::<u32>().unwrap());
       right.push(data[1].parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();
    println!("left: {:?}", left.len());
    println!("right: {:?}", right.len());

    let mut count = 0;
    for i in 0..left.len() {
        let diff = left[i].abs_diff(right[i]);
        println!("Trying {:?} - {:?} = {:?} ", right[i], left[i], diff);
        count += diff;
    }


    count
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 11);
    }
}