use itertools::Itertools;
use winnow::{ascii::dec_uint, combinator::separated, combinator::separated_pair, PResult, Parser};

pub fn run(lines: &str) -> (u32, u32) {
    let equations: Vec<Equation> = lines
        .lines()
        .map(|l| equation.parse(l).expect("{l} failed to parse"))
        .collect();

    validate_equation(&equations[0]);

    let part1 = 0;
    let part2 = 0;

    (part1 as u32, part2 as u32)
}

type Equation = (u32, Vec<u32>);

fn validate_equation(eq: &Equation) -> bool {
    dbg!(eq);

    true
}

fn equation(input: &mut &str) -> PResult<Equation> {
    separated_pair(dec_uint, ": ", operands).parse_next(input)
}

fn operands(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., dec_uint::<_, u32, _>, " ").parse_next(input)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 3749);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 0);
    }
}
