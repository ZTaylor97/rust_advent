use std::{fs::File, io::BufRead, io::BufReader};

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up, 
    Down
}

struct Player {
    x: u32,
    y: u32,
    dir: Direction
}

impl Player {
    // Turn the player to face a new direction based on an input direction of either left or right
    fn turn(&mut self, dir: Direction){
        self.dir = match dir {
            Direction::Left => {
                match self.dir {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Up => Direction::Left
                }
            },
            Direction::Right => {
                match self.dir {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Up => Direction::Right
                }
            },
            _ => panic!("Invalid turn direction input")
        }
    }

    // Walk the player in the direction they're facing until they run out of steps or hit a wall
    fn walk(&mut self, steps: i32) {
    }

}

enum Cell {
    Path,
    Wall,
    Empty
}

struct Map {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>
}

impl Map {
    fn addrow(&mut self, line: &Vec<char>) {
        let mut row: Vec<Cell> = vec![];
        
        for i in 0..self.width{
            let new_cell = match line[i as usize] {
                '.' => Cell::Path,
                '#' => Cell::Wall,
                ' ' => Cell::Empty,
                _ => panic!("Found invalid cell!!!")
            };

            row.push(new_cell);
        }

        self.cells.push(row);
    }
}



// First go through map line by line and create Map using celltype enum
// Make map a width x height 2D array of cells
// Player does not exist on map, simply stores direction and location as an x and y coordinate
// For each instruction, move player cell by cell in direction they are facing, only check the cell directly in front

fn main() {
    let filename = "file.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map = Map {
        width: 150,
        height: 200,
        cells: vec![]
    };

    let player = Player {
        x : 50,
        y : 0,
        dir: Direction::Right
    };

    let mut  loading_state: bool = true;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let linestr: Vec<char> = line.chars().collect();

        // Map loading logic
        if loading_state {
            if line == "" {
                loading_state = false;
                continue;
            }

            map.addrow(&linestr);
        } else { // Map traversal logic

            
        }
    }
}