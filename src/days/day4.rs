use grid::Grid;
use itertools::Itertools;
use std::iter::{Rev, StepBy, Take};
use std::slice::Iter;

const WORD: [u8; 4] = [b'X', b'M', b'A', b'S'];

pub fn run(lines: &str) -> (u32, u32) {
    let grid: Vec<u8> = lines.lines().flat_map(|l| l.as_bytes().to_vec()).collect();
    let grid: Vec<u8> = (0..100).collect();
    let width = lines.lines().next().unwrap().len();
    let grid = Grid::from_vec(grid, width);
    dbg!(&WORD);
    dbg!(&grid);
    dbg!(rows(&grid));
    /*
        dbg!(grid
            .iter_rows()
            .flat_map(|row| SearchIter::new(row.rev()))
            //.flat_map(|row| SearchIter::new(row.rev(), &WORD))
            .filter(|x| *x)
            .count());
    */
    let my_iter = grid.iter_rows().next().unwrap();
    //    dbg!(SearchIter::new(my_iter).filter(|x| *x).count());
    dbg!(SearchIter::new(0..5).filter(|x| *x).count());
    let diagonal2: Vec<_> = diagonal(&grid, Direction::Right, 2).collect();
    dbg!(diagonal2);
    let part1 = 0;
    let part2 = 0;
    (part1 as u32, part2 as u32)
}

fn rows(grid: &Grid<u8>) -> usize {
    grid.iter_col(1).count()
}

enum Direction {
    Left,
    Right,
}

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

struct SearchIter<I> {
    progress: usize,
    grid_iter: I,
    //target: &'a [u8],
    target: [u8; 2],
}

impl<I> SearchIter<I> {
    fn new(iter: I) -> SearchIter<I> {
        dbg!(std::any::type_name::<I>());
        SearchIter {
            progress: 0,
            grid_iter: iter,
            target: [b'h', b'i'],
            //target,
        }
    }
}

impl<I> Iterator for SearchIter<I>
where
    I: Iterator<Item = u8>,
    //I: Iterator<Item: PartialEq>,
{
    type Item = bool;

    fn next(self: &mut Self) -> Option<bool> {
        let want = WORD[self.progress];
        let first = WORD[0];
        let have = self.grid_iter.next();
        if have == None {
            return None;
        }
        //if Some(&want) == have {
        if Some(want) == have {
            self.progress += 1;
            if self.progress == self.target.len() {
                self.progress = 0;
                Some(true)
            } else {
                Some(false)
            }
        } else if Some(first) == have {
            //} else if Some(&first) == have {
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
