pub const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone)]
pub struct PipeDetails {
    // Tuples for relative offset of connected pip
    pub entrance: (i8, i8),
    pub exit: (i8, i8),

    pub pos: (u32, u32),
    pub char: char,
}

#[derive(Debug, Clone)]
pub enum Pipe {
    Start,
    Standard(PipeDetails),
    None,
}

// ((start_x, start_y), pipe_grid)
pub fn parse_input(input: &str) -> ((u32, u32), Vec<Vec<Pipe>>) {
    let mut pipes = Vec::new();
    let mut start = None;

    for (j, row) in input.split('\n').enumerate() {
        let mut pipes_row = Vec::new();
        for (i, char) in row.chars().enumerate() {
            let pipe = match char {
                '.' => Pipe::None,
                'S' => Pipe::Start,
                '|' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (0, -1),
                    exit: (0, 1),
                    char,
                }),
                '-' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (-1, 0),
                    exit: (1, 0),
                    char,
                }),
                '7' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (-1, 0),
                    exit: (0, 1),
                    char,
                }),
                'J' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (0, -1),
                    exit: (-1, 0),
                    char,
                }),
                'L' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (0, -1),
                    exit: (1, 0),
                    char,
                }),
                'F' => Pipe::Standard(PipeDetails {
                    pos: (i as u32, j as u32),
                    entrance: (1, 0),
                    exit: (0, 1),
                    char,
                }),
                _ => panic!("Invalid pipe"),
            };

            if let Pipe::Start = pipe {
                start = Some((i as u32, j as u32));
            }

            pipes_row.push(pipe);
        }

        pipes.push(pipes_row);
    }

    (start.expect("Could not find start pipe"), pipes)
}

pub fn get_next(prev: (u32, u32), curr: (u32, u32), grid: &Vec<Vec<Pipe>>) -> (u32, u32) {
    let directions = match &grid[curr.1 as usize][curr.0 as usize] {
        Pipe::Standard(details) => vec![details.entrance, details.exit],
        Pipe::Start => DIRECTIONS.to_vec(),
        _ => panic!("Empty pipe"),
    };

    for dir in directions {
        let pos = (curr.0 as i32 + dir.0 as i32, curr.1 as i32 + dir.1 as i32);
        if pos.0 < 0 || pos.1 < 0 || pos.1 >= grid.len() as i32 || pos.0 >= grid[0].len() as i32 {
            continue;
        }

        let pos = (pos.0 as u32, pos.1 as u32);
        if pos == prev {
            continue;
        }

        let next = &grid[pos.1 as usize][pos.0 as usize];
        match next {
            Pipe::Standard(pipe_details) => {
                if (pipe_details.entrance.0 == -dir.0 && pipe_details.entrance.1 == -dir.1)
                    || (pipe_details.exit.0 == -dir.0 && pipe_details.exit.1 == -dir.1)
                {
                    return pos;
                }
            }
            Pipe::Start => return pos,
            _ => {
                panic!("Pointing to empty pipe");
            }
        }
    }

    panic!("No connected pipe found");
}
