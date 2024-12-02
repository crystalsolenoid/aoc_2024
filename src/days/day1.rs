use std::collections::HashMap;
use winnow::{
    ascii::{dec_uint, multispace1},
    combinator::separated_pair,
    PResult, Parser,
};

pub fn run(lines: &str) -> (u32, u32) {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        lines.lines().map(|l| line.parse(l).unwrap()).collect();
    left.sort_unstable();
    right.sort_unstable();

    let total_distance = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    let mut keys = right.clone();
    keys.dedup();
    let counts: HashMap<u32, u32> = keys
        .iter()
        .map(|k| {
            let low = right.partition_point(|x| x < k);
            let high = right.partition_point(|x| x <= k);
            let count = high - low;
            (*k, count as u32)
        })
        .collect();

    let similarity_score = left.iter().map(|a| a * counts.get(a).unwrap_or(&0)).sum();

    (total_distance, similarity_score)
}

fn line(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(dec_uint, multispace1, dec_uint).parse_next(input)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 11);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 31);
    }

    #[test]
    fn parse1() {
        let input = "3   4";
        let output = line.parse(input);
        assert_eq!(output, Ok((3, 4)));
    }
}
