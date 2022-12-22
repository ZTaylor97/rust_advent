use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let filename = "file.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let num_pairs = 0;

    let mut stacks: Vec<Vec<char>> = vec![
        vec!['D', 'B', 'J', 'V'],
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['W', 'L', 'F'],
        vec!['S', 'V', 'F', 'M', 'R'],
    ];

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let splitline: Vec<_> = line.split(' ').collect();

        // Extract data from each line
        let mut quantity: i32 = splitline[1].parse().unwrap();
        let mut from: usize = splitline[3].parse().unwrap();
        from -= 1;
        let mut to: usize = splitline[5].parse().unwrap();
        to -= 1;

        let mut stack: Vec<char> = vec![];
        
        quantity = std::cmp::min(quantity, stacks[from].len() as i32);
        let idx = stacks[from].len() - quantity as usize;
        stack = stacks[from].drain(idx..).collect();

        stacks[to].append(&mut stack);
    }

    for mut i in stacks {
        match i.pop() {
            Some(x) => print!("{}", x),
            None => print!(" "),
        }
    }
}
