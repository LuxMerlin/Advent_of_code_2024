use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Day 2 Part 2");
    let mut file = File::open("src/input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("result: {}", get_part2_anwser(&contents));
}


fn get_part2_anwser(contents: &str) -> u32{
    let mut count = 0;
    
    for line in contents.lines() {
       let data: Vec<String> = line.split_whitespace().map(str::to_string).collect();
       
        let is_safe = is_safe_report(&data);
         if is_safe {
            count += 1;
         }else{
            for index in 0..data.len() {
                let new_data = &mut data.clone();
                new_data.remove(index);
                let is_safe = is_safe_report(&new_data);
                if is_safe {
                    count += 1;
                    break;
                }
            }
         }
    }
    count
}

fn is_safe_report(data: &Vec<String>) -> bool{
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

    is_safe
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;

    #[test]
    fn part1_example() {
        let contents = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = get_part2_anwser(contents);
        assert_eq!(result, 4);
    }
}