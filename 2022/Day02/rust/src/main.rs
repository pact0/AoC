
fn main() {
    let filename = "../input";
    let data = parse_input(&read_input(filename));

    let p1 = part1(&data);
    let p2 = part2(&data);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn compute_score(a: u8, b: u8) -> u32 {
  let score = (b - b'X') as u32 + 1;
  if a - b'A' == b - b'X' {
    score + 3
  } else if a - b'A' == (b - b'X' + 1) % 3 {
    score
  } else {
    score + 6
  }
}

fn decide(a: u8, end: u8) -> u8 {
  match end {
    b'X' => ((a - b'A').checked_sub(1).unwrap_or(2)) % 3 + b'X',
    b'Y' => a - b'A' + b'X',
    b'Z' => (a - b'A' + 1) % 3 + b'X',
    _ => unreachable!(),
  }
}

fn part1(data: &Vec<(u8, u8)>) -> u32 {
  data
    .iter()
    .fold(0, |score, (a, b)| score + compute_score(*a, *b))
}

fn part2(data: &Vec<(u8, u8)>) -> u32 {
  data.iter().fold(0, |score, (a, end)| {
    score + compute_score(*a, decide(*a, *end))
  })
}

fn parse_input(input: &str) -> Vec<(u8, u8)> {
  input
    .lines()
    .map(|line| {
      let bytes = line.as_bytes();
      (bytes[0], bytes[2])
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
