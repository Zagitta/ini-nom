use nom::bytes::complete::{tag, take, take_till};
use nom::character::complete::line_ending;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;
use nom_locate::{position, LocatedSpan};

#[derive(Debug, PartialEq, Eq)]
struct IniLine<'a> {
    key: &'a str,
    value: &'a str,
    line: u32,
    key_start: usize,
    value_start: usize,
}

fn ini_line<'a>(input: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, IniLine<'a>> {
    let (remainder, ((key_span, key), (value_span, value))) = terminated(
        separated_pair(
            tuple((position, take_till(|c| c == '='))),
            tag("="),
            tuple((position, take_till(|c| c == '\n' || c == '\r'))),
        ),
        line_ending,
    )(input)?;

    Ok((
        remainder,
        IniLine {
            key: key.fragment(),
            value: value.fragment(),
            line: key_span.location_line(),
            key_start: key_span.location_offset(),
            value_start: value_span.location_offset(),
        },
    ))
}

fn ini_lines<'a>(input: LocatedSpan<&'a str>) -> IResult<LocatedSpan<&'a str>, IniLine<'a>> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line() {
        let (_, line) = ini_line(LocatedSpan::new("foobar=lol\r\n")).unwrap();
        assert_eq!(
            line,
            IniLine {
                key: "foobar",
                value: "lol",
                line: 1,
                key_start: 0,
                value_start: 7,
            }
        )
    }
}
