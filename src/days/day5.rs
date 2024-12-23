use itertools::Itertools;
use winnow::{ascii::dec_uint, combinator::separated, combinator::separated_pair, PResult, Parser};

pub fn run(lines: &str) -> (u32, u32) {
    let binding = lines.lines().chunk_by(|l| l.is_empty());
    let mut input = binding.into_iter();
    let mut rules: Vec<_> = input
        .next()
        .expect("Rules list missing?")
        .1
        .map(|l| rule.parse(l).expect("Rule parsing failed."))
        .collect();
    rules.sort_unstable();
    let _blank_line = input.next();
    let mut updates: Vec<_> = input
        .next()
        .expect("Updates list missing?")
        .1
        .map(|l| update.parse(l).expect("Update parsing failed."))
        .collect();

    let valid_updates: Vec<_> = updates
        .extract_if(|update| validate_update(&rules, &update))
        .collect();
    let mut invalid_updates = updates;

    let part1: usize = valid_updates
        .iter()
        .map(|update| middle_page(&update))
        .sum();

    invalid_updates
        .iter_mut()
        .for_each(|update| fix_update(&rules, update));
    let part2: usize = invalid_updates
        .iter()
        .map(|update| middle_page(&update))
        .sum();

    (part1 as u32, part2 as u32)
}

fn fix_update(rules: &[(usize, usize)], update: &mut [usize]) {
    loop {
        //TODO this could infinite loop?
        let mut violations = update
            .iter()
            .enumerate()
            .combinations(2)
            .map(|pair| ((pair[1].0, pair[0].0), (*pair[1].1, *pair[0].1)))
            .filter(|reverse_pair| rules.binary_search(&reverse_pair.1).is_ok());
        match violations.next() {
            Some(violation) => update.swap(violation.0 .0, violation.0 .1),
            None => break,
        }
    }
}

fn middle_page(update: &[usize]) -> usize {
    let len = update.len();
    let middle = len / 2;
    update[middle]
}

fn validate_update(rules: &[(usize, usize)], update: &[usize]) -> bool {
    update
        .iter()
        .combinations(2)
        .map(|pair| (*pair[1], *pair[0]))
        .all(|reverse_pair| rules.binary_search(&reverse_pair).is_err())
}

fn rule(input: &mut &str) -> PResult<(usize, usize)> {
    separated_pair(dec_uint, "|", dec_uint).parse_next(input)
}

fn update(input: &mut &str) -> PResult<Vec<usize>> {
    separated(1.., dec_uint::<_, usize, _>, ",").parse_next(input)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 143);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, 123);
    }
}
