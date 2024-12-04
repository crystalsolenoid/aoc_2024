use winnow::{
    ascii::{dec_int, multispace1},
    combinator::separated,
    PResult, Parser,
};

pub fn run(lines: &str) -> (u32, u32) {
    let part1 = lines
        .lines()
        .map(|l| line.parse(l).unwrap())
        .filter(|l| report(&l))
        .count();
    (part1 as u32, 0)
}

fn report(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let gradual = diffs.iter().all(|&x| match x.abs() {
        0 => false,
        1..=3 => true,
        _ => false,
    });
    let monodirectional = diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0);
    gradual && monodirectional
}

fn line(input: &mut &str) -> PResult<Vec<i32>> {
    separated(1.., dec_int::<_, i32, _>, " ").parse_next(input)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 2);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 4);
    }

    #[test]
    fn report1() {
        let input = vec![7, 6, 4, 2, 1];
        assert_eq!(report(&input), true);
    }

    #[test]
    fn report2() {
        let input = vec![1, 2, 7, 8, 9];
        assert_eq!(report(&input), false);
    }

    #[test]
    fn parse1() {
        let input = "9 7 6 2 1";
        let output = line.parse(input);
        assert_eq!(output, Ok(vec![9, 7, 6, 2, 1]));
    }
}
