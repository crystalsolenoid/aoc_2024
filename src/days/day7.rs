use itertools::Itertools;
use winnow::{ascii::dec_uint, combinator::separated, combinator::separated_pair, PResult, Parser};

pub fn run(lines: &str) -> (u64, u64) {
    let equations: Vec<Equation> = lines
        .lines()
        .map(|l| equation.parse(l).expect("{l} failed to parse"))
        .collect();

    let part1: u64 = equations
        .iter()
        .filter(|eq| validate_equation_1(eq))
        .map(|eq| eq.0)
        .sum();

    let part2: u64 = equations
        .iter()
        .filter(|eq| validate_equation_2(eq))
        .map(|eq| eq.0)
        .sum();

    (part1 as u64, part2 as u64)
}

type Equation = (u64, Vec<u64>);

#[derive(Clone, Debug)]
enum Op {
    Plus,
    Mul,
    Conc,
}

static OPLIST_1: [Op; 2] = [Op::Mul, Op::Plus];
static OPLIST_2: [Op; 3] = [Op::Conc, Op::Mul, Op::Plus];

fn validate_equation_1(eq: &Equation) -> bool {
    let ops_length = eq.1.len() - 1;
    // start with all mul because it will help with short circuiting in next step?
    (0..ops_length) // weird trick to get all combos with replacement
        .map(|_| OPLIST_1.iter().cloned())
        .multi_cartesian_product()
        // okay weird trick over
        .map(|ops| validate_operations(&eq, &ops))
        .any(|v| v)
}

fn validate_equation_2(eq: &Equation) -> bool {
    let ops_length = eq.1.len() - 1;
    // start with all mul because it will help with short circuiting in next step?
    (0..ops_length) // weird trick to get all combos with replacement
        .map(|_| OPLIST_2.iter().cloned())
        .multi_cartesian_product()
        // okay weird trick over
        .map(|ops| validate_operations(&eq, &ops))
        .any(|v| v)
}

fn validate_operations(eq: &Equation, ops: &[Op]) -> bool {
    let (target, operands) = eq;
    let mut operations = ops.iter();
    let result: u64 = operands
        .iter()
        .copied()
        .reduce(|acc, n| match operations.next() {
            // TODO short circuit if already too big?
            Some(Op::Plus) => acc + n,
            Some(Op::Mul) => acc * n,
            Some(Op::Conc) => todo!(),
            None => panic!("Operations list too short!"),
        })
        .unwrap();
    result == *target
}

fn equation(input: &mut &str) -> PResult<Equation> {
    separated_pair(dec_uint, ": ", operands).parse_next(input)
}

fn operands(input: &mut &str) -> PResult<Vec<u64>> {
    separated(1.., dec_uint::<_, u64, _>, " ").parse_next(input)
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
        assert_eq!(run(EXAMPLE).1, 11387);
    }
}
