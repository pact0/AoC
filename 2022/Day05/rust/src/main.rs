
fn main() {
    let filename = "../input";
    let text = &read_input(filename);

    let p1 = part1(&text);
    let p2 = part2(&text);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(input: &str) -> String {
    let (vals, moves)  = input.split_once("\n\n").unwrap();
    let mut temp = vals.lines().peekable();
    let mut stacks : Vec<Vec<char>>= vec![vec![]; (temp.peek().unwrap().len() + 1) / 4];

    for line in temp.take_while(|l| !l.starts_with(" 1") ) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    }


    moves.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let what: usize = split[1].parse().unwrap();
        let from: usize = split[3].parse().unwrap();
        let where_to: usize = split[5].parse().unwrap();

        for _ in 0..what {
            match stacks[from - 1].pop() {
                Some(val) => stacks[where_to - 1].push(val),
                None =>{}
            }
        }
    });

    let mut result = String::new();
    for i in 0..stacks.len() {
        let t = stacks[i].last().unwrap();
        result.push(t.to_owned());
    }

    return result;
}

fn part2(input: &str) -> String {
    let (vals, moves)  = input.split_once("\n\n").unwrap();
    let mut temp = vals.lines().peekable();
    let mut stacks : Vec<Vec<char>>= vec![vec![]; (temp.peek().unwrap().len() + 1) / 4];

    for line in temp.take_while(|l| !l.starts_with(" 1") ) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    }


    moves.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let what: usize = split[1].parse().unwrap();
        let from: usize = split[3].parse().unwrap();
        let where_to: usize = split[5].parse().unwrap();

        let mut temp_stack: Vec<char> = vec![];

        for _ in 0..what {
            match stacks[from - 1].pop() {
                Some(val) => temp_stack.push(val),
                None =>{}
            }
        }

        for _ in 0..temp_stack.len() {
            match temp_stack.pop() {
                Some(val) => stacks[where_to -1 ].push(val),
                None =>{}
            }
        }

    });

    let mut result = String::new();
    for i in 0..stacks.len() {
        let t = stacks[i].last().unwrap();
        result.push(t.to_owned());
    }

    return result;
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
