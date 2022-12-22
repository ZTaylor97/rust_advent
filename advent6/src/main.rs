use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let filename = "file.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut strlen = 0;

    'outer: for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();


        let linestr: Vec<char> = line.chars().collect();

        let length = linestr.len();

        let mut diffcount = 0;

        // Loop through whole string
        'iloop: for i in 0..length-14 {
            // Loop through local 4
            'jloop : for j in i..i+14 {
                for k in j..i+14 {
                    if k == j {continue;}

                    if linestr[j] == linestr[k] {
                        break 'jloop;
                    }
                }
                diffcount += 1;
            }

            if diffcount == 14 {
                println!("No matches found for {}", &linestr[i..i+14].iter().copied().collect::<String>());
                strlen = i+14;
                break;
            }

            diffcount = 0;
        }
    }

    println!("{}", strlen)
}
