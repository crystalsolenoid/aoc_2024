use grid::Grid;
use itertools::Itertools;

pub fn run(lines: &str) -> (u64, u64) {
    parse(&lines);

    let part1 = 0;
    let part2 = 0;

    (part1 as u64, part2 as u64)
}

type Cell = u8;
type Area = Grid<Cell>;

fn parse(lines: &str) -> Area {
    let mut map: Area = grid::grid![];
    lines.lines().for_each(|l| map.push_row(parse_line(l)));
    map
}

fn parse_line(l: &str) -> Vec<Cell> {
    l.bytes().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 14);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 0);
    }
}
