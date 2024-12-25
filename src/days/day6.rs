use grid::Grid;
use itertools::Itertools;
use std::fmt;

pub fn run(lines: &str) -> (u32, u32) {
    let mut room: Grid<Cell> = grid::grid![];
    let mut guard: (usize, usize, Dir) = (0, 0, Dir::North);
    lines
        .lines()
        .enumerate()
        .for_each(|(y, l)| room.push_row(parse_line(l, y, &mut guard)));
    dbg!(room);
    dbg!(guard);
    let part1 = 0;
    let part2 = 0;
    (part1 as u32, part2 as u32)
}

#[derive(Clone, Copy)]
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

enum Cell {
    Open,
    Barrier,
    Guard(Dir),
    Path,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Open => ".",
                Cell::Barrier => "#",
                Cell::Path => "X",
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
            //            _ => Cell::Guard(c.try_into().map_err("Invalid character found.")),
            _ => byte.try_into().map(|v| Cell::Guard(v)),
        }
    }
}

fn parse_line(l: &str, y: usize, guard: &mut (usize, usize, Dir)) -> Vec<Cell> {
    l.bytes()
        .enumerate()
        .map(|(x, c)| {
            let cell = Cell::try_from(c).unwrap();
            if let Cell::Guard(dir) = cell {
                *guard = (x, y, dir);
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
        assert_eq!(run(EXAMPLE).1, 0);
    }
}
