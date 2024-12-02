use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Day 1 Part 2");
    // open and parse file
    let mut file = File::open("src/input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("result: {}", get_part2_anwser(&contents));
}

// parse the input line by line
// for each line add them to 2 vectors
// sort by smallest to largest
// calculate  how offten the number in the left list appers in the right list and multiply by the number of times it appears
fn get_part2_anwser(contents: &str) -> u32{
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in contents.lines() {
       let data: Vec<String> = line.split_whitespace().map(str::to_string).collect();
       left.push(data[0].parse::<u32>().unwrap());
       right.push(data[1].parse::<u32>().unwrap());
    }

    println!("left: {:?}", left.len());
    println!("right: {:?}", right.len());

    let mut count = 0;
    for i in 0..left.len() {
        let indexes: Vec<_> = right.iter().enumerate().filter(|(_, &num)| num == left[i]).collect();
        count += indexes.len() as u32 * left[i];
     }


    count
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;

    #[test]
    fn part1_example() {
        let contents = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = get_part2_anwser(contents);
        assert_eq!(result, 31);
    }
}