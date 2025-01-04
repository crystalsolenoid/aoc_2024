use grid::Grid;
use std::iter::{Chain, Rev, StepBy, Take};
use std::slice::Iter;

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

pub fn run(lines: &str) -> (u64, u64) {
    let grid: Vec<u8> = lines.lines().flat_map(|l| l.as_bytes().to_vec()).collect();
    //    let grid: Vec<u8> = (0..100).collect();
    let width = lines.lines().next().unwrap().len();
    let grid = Grid::from_vec(grid, width);
    let part1 = part_1(&grid);
    let part2 = grid.count_x_mas();
    (part1 as u64, part2 as u64)
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
        .map(|i| grid.diagonal(Direction::Left, i))
        .chain(((1 - h as i32)..w as i32).map(|i| grid.diagonal(Direction::Right, i)))
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

trait Occupancy {
    fn count_x_mas(&self) -> usize;
}

impl Occupancy for Grid<u8> {
    fn count_x_mas(&self) -> usize {
        self.indexed_iter()
            .filter(|(_, v)| **v == b'A')
            .filter(|((x, y), _)| *x > 0 && *y > 0)
            .filter(|((x, y), _)| {
                let nw = self.get(x - 1, y - 1);
                let se = self.get(x + 1, y + 1);
                match nw.zip(se) {
                    Some((b'M', b'S')) => true,
                    Some((b'S', b'M')) => true,
                    _ => false,
                }
            })
            .filter(|((x, y), _)| {
                let ne = self.get(x + 1, y - 1);
                let sw = self.get(x - 1, y + 1);
                match ne.zip(sw) {
                    Some((b'M', b'S')) => true,
                    Some((b'S', b'M')) => true,
                    _ => false,
                }
            })
            .count()
    }
}

trait Diagonal<T> {
    fn diagonal<'a>(&'a self, dir: Direction, x: i32) -> Take<StepBy<Iter<'a, T>>>;
}

impl<T> Diagonal<T> for Grid<T> {
    fn diagonal<'a>(&'a self, dir: Direction, x: i32) -> Take<StepBy<Iter<'a, T>>> {
        // TODO this only works with the default row-major order right now...
        let step = match dir {
            Direction::Left => self.cols() - 1,
            Direction::Right => self.cols() + 1,
        };
        let (start, take) = match x {
            a if a >= self.cols() as i32 => {
                panic!("{a} is too big, exceeds column grid bound {}", self.cols())
            }
            a if -a >= self.rows() as i32 => panic!(
                "{a} is too small, its negative exceeds row grid bound {}",
                self.rows()
            ),
            // starting from the top
            a if a >= 0 => {
                let x = x as usize;
                let start = x;
                assert!((0..self.cols()).contains(&x));
                let take = match dir {
                    Direction::Left => x + 1,
                    Direction::Right => self.cols() - x,
                };
                (start, take)
            }
            // starting from the side
            _ => {
                let y = (-x) as usize;
                let take = self.rows();
                let start = match dir {
                    Direction::Left => y * self.cols() + self.rows() - 1,
                    Direction::Right => y * self.cols(),
                };
                (start, take)
            }
        };
        let mut diag = self.iter();
        if start > 0 {
            let _ = diag.nth(start - 1);
        }
        diag.step_by(step).take(take)
    }
}

struct SearchIter<'a, I: Iterator> {
    progress: usize,
    grid_iter: I,
    target: &'a [u8],
}

impl<I> SearchIter<'_, I>
where
    I: Iterator,
{
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
        assert_eq!(run(EXAMPLE).1, 9);
    }
}
