use grid::Grid;
use itertools::Itertools;
use std::iter::{Rev, StepBy, Take};
use std::slice::Iter;

const WORD: [u8; 4] = [b'X', b'M', b'A', b'S'];

pub fn run(lines: &str) -> (u32, u32) {
    let grid: Vec<u8> = lines.lines().flat_map(|l| l.as_bytes().to_vec()).collect();
    //    let grid: Vec<u8> = (0..100).collect();
    let width = lines.lines().next().unwrap().len();
    let grid = Grid::from_vec(grid, width);
    dbg!(&WORD);
    dbg!(&grid);
    let horizontal_count = &grid
        .iter_rows()
        .flat_map(|i| {
            let reverse = SearchIter::new(i.clone().rev(), &WORD);
            let forwards = SearchIter::new(i, &WORD);
            reverse.chain(forwards)
        })
        .filter(|x| *x)
        .count();
    let vertical_count = &grid
        .iter_cols()
        .flat_map(|i| {
            let reverse = SearchIter::new(i.clone().rev(), &WORD);
            let forwards = SearchIter::new(i, &WORD);
            reverse.chain(forwards)
        })
        .filter(|x| *x)
        .count();
    dbg!(horizontal_count + vertical_count);
    let diagonal2: Vec<_> = diagonal(&grid, Direction::Right, 2).collect();
    dbg!(diagonal2);
    let part1 = 0;
    let part2 = 0;
    (part1 as u32, part2 as u32)
}

enum Direction {
    Left,
    Right,
}

// TODO include diagonals that start from the side
fn diagonal<'a, T>(grid: &'a Grid<T>, dir: Direction, x: usize) -> Take<StepBy<Iter<'a, T>>> {
    // TODO this only works with the default row-major order right now...
    assert!((0..grid.cols()).contains(&x));
    let (step, take) = match dir {
        Direction::Left => (grid.cols() - 1, x + 1),
        Direction::Right => (grid.cols() + 1, grid.cols() - x),
    };
    let mut diag = grid.iter();
    let _ = diag.nth(x - 1);
    diag.step_by(step).take(take)
}

//type GridSlice<'a> = Rev<StepBy<Iter<'a, u8>>>;

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
        let want = WORD[self.progress];
        let first = WORD[0];
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
        assert_eq!(run(EXAMPLE).1, 0);
    }
}
