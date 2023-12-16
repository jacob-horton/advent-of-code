use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    VertSplitter,
    HorizSplitter,
    TlBrMirror,
    BlTrMirror,
}

pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let mut grid = Vec::new();

    for line in input.trim().split('\n') {
        let mut row = Vec::new();
        for char in line.chars() {
            let tile = match char {
                '.' => Tile::Empty,
                '|' => Tile::VertSplitter,
                '-' => Tile::HorizSplitter,
                '\\' => Tile::TlBrMirror,
                '/' => Tile::BlTrMirror,
                _ => panic!("Unknown tile"),
            };

            row.push(tile);
        }

        grid.push(row)
    }

    grid
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Beam {
    pub pos: (i32, i32),
    pub dir: (i32, i32),
}

fn out_of_bounds(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 < 0 || pos.1 < 0 || pos.0 >= width || pos.1 >= height
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn get_energised_tiles(grid: &Vec<Vec<Tile>>, starting_beam: Beam) -> u32 {
    let mut beams = vec![starting_beam];

    let mut previous_beam_positions = HashSet::new();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    while !beams.is_empty() {
        let mut beams_to_delete: Vec<Beam> = Vec::new();
        let mut beams_to_add: Vec<Beam> = Vec::new();

        for beam in beams.iter_mut() {
            if out_of_bounds(beam.pos, width, height) {
                beams_to_delete.push(beam.to_owned());
                continue;
            }

            if previous_beam_positions.contains(beam) {
                beams_to_delete.push(beam.to_owned());
                continue;
            }

            previous_beam_positions.insert(beam.clone());

            let mut new_beam = None;
            match grid[beam.pos.1 as usize][beam.pos.0 as usize] {
                Tile::Empty => {
                    // While the next tile is empty, increase pos (saves ~30ms in release)
                    let mut next_pos = add(beam.pos, beam.dir);
                    while !out_of_bounds(next_pos, width, height)
                        && grid[next_pos.1 as usize][next_pos.0 as usize] == Tile::Empty
                    {
                        beam.pos = add(beam.pos, beam.dir);
                        next_pos = add(beam.pos, beam.dir);

                        previous_beam_positions.insert(beam.clone());
                    }
                }
                Tile::TlBrMirror => (beam.dir.0, beam.dir.1) = (beam.dir.1, beam.dir.0),
                Tile::BlTrMirror => (beam.dir.0, beam.dir.1) = (-beam.dir.1, -beam.dir.0),
                Tile::VertSplitter => {
                    // Moving horizontally
                    if beam.dir.0.abs() != 0 {
                        beam.dir = (0, 1);
                        new_beam = Some(Beam {
                            pos: beam.pos,
                            dir: (0, -1),
                        });
                    }
                }
                Tile::HorizSplitter => {
                    // Moving vertically
                    if beam.dir.1 != 0 {
                        beam.dir = (1, 0);
                        new_beam = Some(Beam {
                            pos: beam.pos,
                            dir: (-1, 0),
                        });
                    }
                }
            }

            beam.pos = add(beam.pos, beam.dir);
            if let Some(mut new_beam) = new_beam {
                new_beam.pos = add(new_beam.pos, new_beam.dir);
                beams_to_add.push(new_beam);
            }
        }

        for beam in beams_to_delete.clone().into_iter() {
            beams.retain(|b| b != &beam);
        }

        for beam in beams_to_add {
            beams.push(beam);
        }
    }

    // Energised tiles are unique positions on the grid, ignoring direction of beam
    previous_beam_positions
        .iter()
        .map(|b| b.pos)
        .collect::<HashSet<_>>()
        .len() as u32
}
