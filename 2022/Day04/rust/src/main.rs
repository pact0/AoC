
fn main() {
    let filename = "../input";
    let text = &read_input(filename);

    let p1 = part1(&text);
    let p2 = part2(&text);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(input: &str) -> i32 {
  input
    .lines()
    .fold(0, |total, line| {
            let vals :Vec<i32> = line.split(",").flat_map(|s| {
                let strs:Vec<&str> = s.split("-").collect();
                let vals:Vec<i32> = strs.iter().map(|val| {
                    val.parse().unwrap()
                }).collect();

                return [vals[0], vals[1]] // how the fuck do you collect into an array XD
            }
            ).collect();

                if vals[0] < vals[2] && vals[1] < vals[3] || vals[2] < vals[0] && vals[3] < vals[1]{
                    return total + 1;
                }

            total
    })
}

fn part2(input: &str) -> usize {
  input.lines()
    .map(|l| l.split(',').collect())
    .map(
      |s1: Vec<&str>| s1.iter().map(
        |part| part.split('-').map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>()
      ).collect::<Vec<Vec<i32>>>()
    )
    .filter( |v| {
      return (v[0][0] <= v[1][1] && v[0][1] >= v[1][1]) || (v[1][0] <= v[0][1] && v[1][1] >= v[0][1])
    })
    .count()
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
