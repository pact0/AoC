
fn main() {
    let filename = "../input";
    let data = read_input(&filename);
    let trimmed = data.trim();
    let split_data: Vec<&str> = trimmed.split("\n\n").collect();

    let p1 = part1(&split_data);
    let p2 = part2(&split_data);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(input: &Vec<&str>) -> i32 {
    let v: Vec<i32> = input
        .iter()
        .map(|v| {
            v.split("\n")
                .map(|v| v.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    let largest = v.iter().max().unwrap();
    return *largest;
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut v: Vec<i32> = input
        .iter()
        .map(|v| {
            v.split("\n")
                .map(|v| v.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    v.sort_by(|a, b| b.cmp(a));
    return v[0..3].iter().sum::<i32>();
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
