
fn main() {
    let filename = "../input";
    let text = &read_input(filename);

    let p1 = part1(&text);
    let p2 = part2(&text);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(input: &str) -> usize {
    input.as_bytes()
    .windows(4)
    .position(|window| {
        window
            .iter()
            .enumerate()
            .all(|(idx, c)| !window[..idx].contains(c))
    })
    .unwrap()
    + 4
}

fn part2(input: &str) -> usize {
    input.as_bytes()
    .windows(14)
    .position(|window| {
        window
            .iter()
            .enumerate()
            .all(|(idx, c)| !window[..idx].contains(c))
    })
    .unwrap()
    + 14
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
