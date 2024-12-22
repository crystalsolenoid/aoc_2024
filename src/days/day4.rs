use grid::Grid;
use itertools::Itertools;
use std::iter::{Chain, Map, Rev, StepBy, Take};
use std::ops::Range;
use std::slice::Iter;

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
const MAS: [u8; 3] = [b'M', b'A', b'S'];

pub fn run(lines: &str) -> (u32, u32) {
    let grid: Vec<u8> = lines.lines().flat_map(|l| l.as_bytes().to_vec()).collect();
    //    let grid: Vec<u8> = (0..100).collect();
    let width = lines.lines().next().unwrap().len();
    let grid = Grid::from_vec(grid, width);
    let part1 = part_1(&grid);
    let part2 = 0;
    (part1 as u32, part2 as u32)
}

fn part_1(grid: &Grid<u8>) -> usize {
    let w = grid.rows();
    let h = grid.cols();
    let orthog_count = &grid
        .iter_rows()
        .chain(grid.iter_cols())
        .flat_map(|i| search_both_ways(i, &XMAS))
        .filter(|x| *x)
        .count();
    let diagonal_count = ((1 - h as i32)..w as i32)
        .map(|i| diagonal(&grid, Direction::Left, i))
        .chain(((1 - h as i32)..w as i32).map(|i| diagonal(&grid, Direction::Right, i)))
        .flat_map(|i| search_both_ways(i, &XMAS))
        .filter(|x| *x)
        .count();
    orthog_count + diagonal_count
}

fn search_both_ways<'a, I>(
    iter: I,
    word: &'a [u8],
) -> Chain<SearchIter<'a, Rev<I>>, SearchIter<'a, I>>
where
    I: Iterator<Item = &'a u8> + Clone + DoubleEndedIterator,
{
    let reverse = SearchIter::new(iter.clone().rev(), word);
    let forwards = SearchIter::new(iter, word);
    reverse.chain(forwards)
}

enum Direction {
    Left,
    Right,
}

fn diagonal<'a, T>(grid: &'a Grid<T>, dir: Direction, x: i32) -> Take<StepBy<Iter<'a, T>>> {
    // TODO this only works with the default row-major order right now...
    let step = match dir {
        Direction::Left => grid.cols() - 1,
        Direction::Right => grid.cols() + 1,
    };
    let (start, take) = match x {
        a if a >= grid.cols() as i32 => {
            panic!("{a} is too big, exceeds column grid bound {}", grid.cols())
        }
        a if -a >= grid.rows() as i32 => panic!(
            "{a} is too small, its negative exceeds row grid bound {}",
            grid.rows()
        ),
        // starting from the top
        a if a >= 0 => {
            let x = x as usize;
            let start = x;
            assert!((0..grid.cols()).contains(&x));
            let take = match dir {
                Direction::Left => x + 1,
                Direction::Right => grid.cols() - x,
            };
            (start, take)
        }
        // starting from the side
        _ => {
            let y = (-x) as usize;
            let take = grid.rows();
            let start = match dir {
                Direction::Left => y * grid.cols() + grid.rows() - 1,
                Direction::Right => y * grid.cols(),
            };
            (start, take)
        }
    };
    let mut diag = grid.iter();
    if start > 0 {
        let _ = diag.nth(start - 1);
    }
    diag.step_by(step).take(take)
}

struct SearchIter<'a, I> {
    progress: usize,
    grid_iter: I,
    target: &'a [u8],
}

impl<I> SearchIter<'_, I> {
    fn new(iter: I, target: &[u8]) -> SearchIter<I> {
        SearchIter {
            progress: 0,
            grid_iter: iter,
            target,
        }
    }
}

impl<'a, I> Iterator for SearchIter<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = bool;

    fn next(self: &mut Self) -> Option<bool> {
        let want = self.target[self.progress];
        let first = self.target[0];
        let have = self.grid_iter.next();
        if have == None {
            return None;
        }
        if Some(&want) == have {
            //        if Some(want) == have {
            self.progress += 1;
            if self.progress == self.target.len() {
                self.progress = 0;
                Some(true)
            } else {
                Some(false)
            }
        //} else if Some(first) == have {
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
        assert_eq!(run(EXAMPLE).1, 9);
    }
}
