use std::{fs::File, io::BufRead, io::BufReader};

struct advent_file {
    size : usize,
    files: Vec<Self>
}

fn main() {
    let filename = "file.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let dir = advent_file {
        size: 20,
        files: vec![]
    };

    let another_dir = advent_file {
        size: 100,
        files: vec![dir]
    };

    let another_another_dir = advent_file {
        size: 200,
        files: vec![another_dir]
    };



    for (_, line) in reader.lines().enumerate() {

    }
}

