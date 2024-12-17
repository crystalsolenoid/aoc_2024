use grid::Grid;
use itertools::Itertools;
use std::iter::{Flatten, StepBy, Take};

const WORD: [u8; 4] = [b'X', b'M', b'A', b'S'];

pub fn run(lines: &str) -> (u32, u32) {
    let grid: Vec<u8> = lines.lines().flat_map(|l| l.as_bytes().to_vec()).collect();
    let grid: Vec<u8> = (0..100).collect();
    let width = lines.lines().next().unwrap().len();
    let grid = Grid::from_vec(grid, width);
    dbg!(&WORD);
    dbg!(&grid);
    dbg!(rows(&grid));
    dbg!(grid
        .iter_rows()
        .flat_map(|row| SearchIter::new(row, &WORD))
        .filter(|x| *x)
        .count());
    let diagonal2: Vec<_> = diagonal_right(&grid, 2).collect();
    dbg!(diagonal2);
    let part1 = 0;
    let part2 = 0;
    (part1 as u32, part2 as u32)
}

fn rows(grid: &Grid<u8>) -> usize {
    grid.iter_col(1).count()
}

fn diagonal_right<'a, T>(grid: &'a Grid<T>, x: usize) -> Take<StepBy<std::slice::Iter<'a, T>>> {
    // TODO this only works with the default row-major order right now...
    assert!((0..grid.cols()).contains(&x));
    let mut diag = grid.iter();
    let _ = diag.nth(x - 1);
    diag.step_by(grid.cols() + 1).take(grid.cols() - x)
}

struct SearchIter<'a> {
    //    grid: &'a Grid<u8>,
    progress: usize,
    grid_iter: StepBy<std::slice::Iter<'a, u8>>,
    target: &'a [u8],
}

impl SearchIter<'_> {
    fn new<'a>(iter: StepBy<std::slice::Iter<'a, u8>>, target: &'a [u8]) -> SearchIter<'a> {
        SearchIter {
            progress: 0,
            grid_iter: iter,
            target,
        }
    }
}

impl Iterator for SearchIter<'_> {
    type Item = bool;
    fn next(self: &mut Self) -> Option<bool> {
        let want = WORD[self.progress];
        let first = WORD[0];
        let have = self.grid_iter.next();
        if have == None {
            return None;
        }
        if Some(&want) == have {
            self.progress += 1;
            if self.progress == self.target.len() {
                self.progress = 0;
                Some(true)
            } else {
                Some(false)
            }
        } else if Some(&first) == have {
            self.progress = 1;
            Some(false)
        } else {
            self.progress = 0;
            Some(false)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 18);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 0);
    }
}