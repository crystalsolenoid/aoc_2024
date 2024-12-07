use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, preceded, repeat, rest, separated_pair},
    stream::AsChar,
    token::{one_of, take_until},
    PResult, Parser,
};

pub fn run(lines: &str) -> (u32, u32) {
    let part1 = parse_muls
        .parse(lines)
        .unwrap()
        .iter()
        .map(|(a, b)| a * b)
        .sum();
    (part1, 0)
}

fn mul(input: &mut &str) -> PResult<(u32, u32)> {
    preceded("mul", args).parse_next(input)
}

fn args(input: &mut &str) -> PResult<(u32, u32)> {
    delimited('(', pair, ')').parse_next(input)
}

fn pair(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(num, ',', num).parse_next(input)
}

fn take_digits<'s>(input: &mut &'s str) -> PResult<&'s str> {
    let mut parser = repeat(1..=3, one_of(AsChar::is_dec_digit))
        .map(|()| ())
        .take();
    parser.parse_next(input)
}

fn num(input: &mut &str) -> PResult<u32> {
    take_digits.and_then(dec_uint).parse_next(input)
}

fn discard_junk(input: &mut &str) -> PResult<()> {
    take_until(0.., "mul").void().parse_next(input)
}

fn potential_mul(input: &mut &str) -> PResult<Option<(u32, u32)>> {
    alt((
        mul.map(|p: (u32, u32)| Some(p)),
        "mul".value(None),
        discard_junk.value(None),
    ))
    .parse_next(input)
}

fn parse_muls(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    let potentials: Vec<Option<(u32, u32)>> = repeat(0.., potential_mul).parse_next(input)?;
    rest.parse_next(input)?;
    let out: Vec<(u32, u32)> = potentials.iter().cloned().flatten().collect();
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1() {
        assert_eq!(run(EXAMPLE).0, 161);
    }

    #[test]
    fn part2() {
        assert_eq!(run(EXAMPLE).1, todo!());
    }

    #[test]
    fn parse_mul1() {
        let input = "mul(44,46)";
        let output = mul.parse(input);
        assert_eq!(output, Ok((44, 46)));
    }

    #[test]
    fn parse_mul2() {
        let input = "mul(1144,46)";
        let output = mul.parse(input);
        assert!(output.is_err());
    }

    #[test]
    fn parse_digits() {
        let input = "1234";
        let output = num.parse(input);
        assert!(output.is_err());
    }

    #[test]
    fn parse_digits_str() {
        let mut input = "1234";
        let output = take_digits.parse_next(&mut input);
        assert_eq!(output, Ok("123"));
    }

    #[test]
    fn parse_ignore_invalid() {
        let mut input = "nul(4*mul(44,46)";
        let output = discard_junk.parse_next(&mut input);
        assert_eq!(output, Ok(()));
        assert_eq!(input, "mul(44,46)");
    }

    #[test]
    fn parse_skip_invalid_mul() {
        let mut input = "mul(4*mul(44,46)";
        let output = potential_mul.parse_next(&mut input);
        assert_eq!(output, Ok(None));
        assert_eq!(input, "(4*mul(44,46)");
    }

    #[test]
    fn parse_multiple_mul() {
        let input = "mul(4*mul(44,46)";
        let output = parse_muls.parse(input);
        assert_eq!(output, Ok(vec![(44, 46)]));
    }
}
