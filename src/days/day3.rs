use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, peek, preceded, repeat, rest, separated_pair},
    stream::AsChar,
    token::{one_of, take_until},
    PResult, Parser, Stateful,
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

#[derive(Debug)]
struct State<'s>(&'s mut bool);

impl<'s> State<'s> {
    fn found_do(&mut self) {
        *self.0 = true;
    }
    fn found_dont(&mut self) {
        *self.0 = false;
    }
}

type Stream<'is> = Stateful<&'is str, State<'is>>;

fn found_do<'s>(i: &mut Stream<'s>) -> PResult<&'s str> {
    let out = "do()".parse_next(i);
    // only update the state if the parsing actually succeeds, or else it'll get
    // messed with when checked during a branch like in alt()
    if out.is_ok() {
        i.state.found_do();
    }
    out
}

fn found_dont<'s>(i: &mut Stream<'s>) -> PResult<&'s str> {
    let out = "don't()".parse_next(i);
    // only update the state if the parsing actually succeeds, or else it'll get
    // messed with when checked during a branch like in alt()
    if out.is_ok() {
        i.state.found_dont();
    }
    out
}

fn parse_toggled_muls(input: &mut Stream) -> PResult<Vec<(u32, u32)>> {
    let potentials: Vec<Option<(u32, u32)>> = repeat(0.., potential_functions).parse_next(input)?;
    rest.parse_next(input)?;
    let out: Vec<(u32, u32)> = potentials.iter().cloned().flatten().collect();
    Ok(out)
}

fn potential_functions(input: &mut Stream) -> PResult<Option<(u32, u32)>> {
    alt((
        found_dont.value(None),
        found_do.value(None),
        //        mul.map(|p: (u32, u32)| Some(p)),
        "mul".value(None),
        //        discard_junk.value(None),
    ))
    .parse_next(input)
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
        assert_eq!(run(EXAMPLE).1, 48);
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

    #[test]
    fn parse_do() {
        let input = "do()";
        let mut state = false;
        let input = Stream {
            input,
            state: State(&mut state),
        };
        let output = found_do.parse(input).unwrap();
        assert_eq!(state, true);
    }

    #[test]
    fn parse_failed_do() {
        let input = "doo()";
        let mut state = false;
        let input = Stream {
            input,
            state: State(&mut state),
        };
        let output = alt((found_do, "doo()")).parse(input).unwrap();
        assert_eq!(state, false);
    }
}
