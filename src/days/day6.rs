use grid::Grid;
use itertools::Itertools;
use std::fmt;

pub fn run(lines: &str) -> (u32, u32) {
    let mut room: Grid<Cell> = grid::grid![];
    // TODO how to initialize guard for the first time in function? Probably some
    // Option type stuff...
    let mut guard: Guard = Guard::new(0, 0, Dir::North);
    lines
        .lines()
        .enumerate()
        .for_each(|(y, l)| room.push_row(parse_line(l, y, &mut guard)));

    //add_obstacle(3, 6, &mut room);

    let mut outcome = StepOutcome::NotDone;
    while outcome == StepOutcome::NotDone {
        outcome = guard.movement(&mut room);
    }

    if outcome == StepOutcome::FoundLoop {
        dbg!("found a loop");
    }

    let part1 = room
        .iter()
        .filter(|&cell| match *cell {
            Cell::Path(_, _) => true,
            Cell::Guard(_) => true,
            _ => false,
        })
        .count();
    dbg!(&room);

    let part2 = 0;
    (part1 as u32, part2 as u32)
}

fn add_obstacle(x: usize, y: usize, r: &mut Grid<Cell>) {
    match r.get_mut(y, x) {
        Some(Cell::Guard(_)) => panic!(),
        Some(Cell::Barrier) => panic!(),
        Some(c) => *c = Cell::NewBarrier,
        None => panic!(),
    };
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    d: Dir,
}

#[derive(Debug)]
enum StepErr {
    Loop,
    Barrier,
    Edge,
}

#[derive(PartialEq)]
enum StepOutcome {
    NotDone,
    LeftArea,
    FoundLoop,
}

impl Guard {
    fn new(x: usize, y: usize, d: Dir) -> Self {
        Guard { x, y, d }
    }

    fn new_path(&self, r: &Grid<Cell>) -> Cell {
        let current_path = match self.d {
            Dir::North | Dir::South => Cell::Path(false, true),
            Dir::West | Dir::East => Cell::Path(true, false),
        };
        match r[(self.y, self.x)] {
            Cell::Guard(a) => Cell::Guard(a),
            Cell::Open => current_path,
            Cell::Path(h, v) => {
                if let Cell::Path(c_h, c_v) = current_path {
                    Cell::Path(h || c_h, v || c_v)
                } else {
                    panic!()
                }
            }
            _ => todo!("{:?}", self.d),
        }
    }

    fn movement(&mut self, r: &mut Grid<Cell>) -> StepOutcome {
        match self.try_step(r) {
            Err(StepErr::Edge) => {
                r[(self.y, self.x)] = self.new_path(r);
                StepOutcome::LeftArea
            }
            Err(StepErr::Barrier) => self.turn(r), // loop!
            Err(StepErr::Loop) => StepOutcome::FoundLoop,
            Ok(_) => StepOutcome::NotDone,
        }
    }

    fn turn(&mut self, r: &mut Grid<Cell>) -> StepOutcome {
        self.d = match self.d {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        };
        let path = Cell::Path(true, true);
        let old_cell = r[(self.y, self.x)];
        r[(self.y, self.x)] = path;
        match old_cell == path {
            true => StepOutcome::FoundLoop,
            false => StepOutcome::NotDone,
        }
    }

    fn try_step(&mut self, r: &mut Grid<Cell>) -> Result<(usize, usize), StepErr> {
        match self.pointing_towards() {
            None => Err(StepErr::Edge),
            Some((x, y)) => match r.get(y, x) {
                None => Err(StepErr::Edge),
                Some(Cell::Barrier) => Err(StepErr::Barrier),
                Some(Cell::NewBarrier) => Err(StepErr::Barrier),
                _ => {
                    let path = self.new_path(r);
                    let old_cell = r[(self.y, self.x)];
                    r[(self.y, self.x)] = path;
                    self.x = x;
                    self.y = y;
                    match path {
                        Cell::Path(true, true) => Ok((y, x)),
                        Cell::Path(_, _) if path == old_cell => Err(StepErr::Loop),
                        _ => Ok((y, x)),
                    }
                }
            },
        }
    }

    fn pointing_towards(&self) -> Option<(usize, usize)> {
        match self.d {
            Dir::North => Some(self.x).zip(self.y.checked_sub(1)),
            Dir::East => Some((self.x + 1, self.y)),
            Dir::South => Some((self.x, self.y + 1)),
            Dir::West => self.x.checked_sub(1).zip(Some(self.y)),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&Dir as Into<&str>>::into(self))
    }
}

impl TryFrom<u8> for Dir {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'^' => Ok(Dir::North),
            b'>' => Ok(Dir::East),
            b'v' => Ok(Dir::South),
            b'<' => Ok(Dir::West),
            _ => Err("Invalid character."),
        }
    }
}

impl Into<&str> for &Dir {
    fn into(self) -> &'static str {
        match self {
            Dir::North => "^",
            Dir::East => ">",
            Dir::South => "v",
            Dir::West => "<",
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Open,
    Barrier,
    NewBarrier,
    Guard(Dir),
    Path(bool, bool), // (horizontal, vertical)
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Open => ".",
                Cell::Barrier => "#",
                Cell::NewBarrier => "O",
                Cell::Path(true, false) => "-",
                Cell::Path(false, true) => "|",
                Cell::Path(true, true) => "+",
                Cell::Path(false, false) => panic!("invald state"),
                Cell::Guard(d) => d.into(),
            }
        )
    }
}

impl TryFrom<u8> for Cell {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'.' => Ok(Cell::Open),
            b'#' => Ok(Cell::Barrier),
            _ => byte.try_into().map(|v| Cell::Guard(v)),
        }
    }
}

fn parse_line(l: &str, y: usize, guard: &mut Guard) -> Vec<Cell> {
    l.bytes()
        .enumerate()
        .map(|(x, c)| {
            let cell = Cell::try_from(c).unwrap();
            if let Cell::Guard(dir) = cell {
                *guard = Guard::new(x, y, dir);
            }
            cell
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 41);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 6);
    }
}
