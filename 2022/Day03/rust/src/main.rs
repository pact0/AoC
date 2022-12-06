use std::str::Lines;
use std::collections::HashSet;

fn main() {
    let filename = "../input";
    let text = &read_input(filename);
    let data = parse_input(text);

    let p1 = part1(&data);
    let p2 = part2(text.lines());

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn compute_score(a: &String, b: &String) -> i32 {
   let hm: HashSet<char> = a.chars().map(char::from).collect(); 
   let mut hm2: HashSet<char> = b.chars().map(char::from).collect(); 
    let mut result = 0;
    
    for v in hm.iter() {
        if let Some(found) = hm2.take(v) {
            if found >= 'a' && found <= 'z' {
                result += (found as u32) - 96;
            } else if found >= 'A' && found <= 'Z' {
                result += (found as u32) - 64 + 26;
            }
        }
    }
    return result as i32;
}

fn bit_encode(a: char) -> u64 {
    let code = a as u32;
    let out = if code > 'Z' as u32 {
        1 + code - 'a' as u32
    } else {
        27 + code - 'A' as u32
    };

   1u64 << out 
}

fn part1(data : &Vec<(String, String)>) -> i32 {
    data
    .iter()
    .fold(0, |score, (a, b)| score + compute_score(&a.to_string(), &b.to_string()))
}

fn vectorize_string(input: &str) -> u64 {
    input.chars().map(bit_encode).reduce(|i, j| i | j).unwrap()
}

fn find_badge(group: &[&str]) -> u64 {
    if let [x, y, z] = group {
        return vectorize_string(x) & vectorize_string(y) & vectorize_string(z);
    }
    panic!("Wrong sized chunk!")
}

fn one_hot_to_num(input: u64) -> i32 {
    f64::log2(input as f64) as i32
}

fn part2(input: Lines) -> i32 {
    input
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_badge)
        .map(one_hot_to_num)
        .sum()
}

fn parse_input(input: &str) -> Vec<(String, String)> {
  input
    .lines()
    .map(|line| {
        let len = line.len();
      (line[0..(len/2)].to_string(), line[(len/2)..len].to_string())
    })
    .collect()
}

fn read_input(path: &str) -> String {
    let result = std::fs::read_to_string(path);
    return match result {
        Ok(data) => data,
        Err(e) => {
            println!("error fetching file at {path}: {e}");
            std::process::exit(0)
        }
    };
}
