use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Day 2 Part 1");
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
    let mut count = 0;
    
    for line in contents.lines() {
       let data: Vec<String> = line.split_whitespace().map(str::to_string).collect();
       
       let mut is_decending = false;
       let mut is_safe = true;
       for i in 0..data.len() {
          if i == data.len() - 1 {
            break;
          }
          let first = data[i].parse::<u32>().unwrap();
          let next = data[i+1].parse::<u32>().unwrap();
          
          if  i == 0 {
            is_decending = next < first;
          } else {
            if is_decending && next > first {
                is_safe = false;
              break;
            }else if !is_decending && next < first {
                is_safe = false;
              break;
            }
          }
          let diff = first.abs_diff(next);
          if diff == 0 || diff > 3 {
            is_safe = false;
            continue; 
          }
       }

         if is_safe {
            count += 1;
         }
    }


    count
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 2);
    }
}