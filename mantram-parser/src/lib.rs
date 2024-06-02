use std::str::FromStr;
use std::string::ToString;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::line_ending;
use nom::combinator::{map_res, opt, recognize, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use strum_macros::EnumString;
// use wasm_bindgen::prelude::*;

pub fn parse_mantram_string(input: &str) -> Result<Vec<Character>, String> {
    let (_, chars) = separated_list0(tag("\n\n"), parse_triples)(input)
        .or(Err("parsing failed".to_string()))?;

    Ok(chars.into_iter().flatten().collect())
}

fn parse_triples(input: &str) -> IResult<&str, Vec<Character>> {
    let (input, (subs, hanzis, pinyins)) =
        tuple((parse_line, parse_line, opt(parse_line)))(input)?;

    if let Some(pinyins) = pinyins {
        todo!()
    }

    Ok((input, vec![]))
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    preceded(opt(line_ending), take_until("\n"))(input)
}

fn punc(input: &str) -> IResult<&str, Character> {
    let (input, p) = map_res(take(1usize), Punctuation::from_str)(input)?;

    Ok((input, Character::Punc(p)))
}

fn context(input: &str) -> IResult<&str, Character> {
    let (input, s) =
        recognize(delimited(tag("（"), take_until("）"), tag("）")))(input)?;

    Ok((input, Character::Context(s.to_string())))
}

fn linebreak(input: &str) -> IResult<&str, Character> {
    value(
        Character::Linebreak,
        alt((tag("<br/>"), tag("<br />"), tag("<br>"))),
    )(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Character {
    Hanzi(Hanzi),
    Punc(Punctuation),
    Context(String),
    Linebreak,
}

#[derive(Debug, Clone, PartialEq)]
struct Hanzi {
    char: String,
    sub: String,
    pinyin: Option<String>,
}

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
#[non_exhaustive]
enum Punctuation {
    #[strum(serialize = "。")]
    Period,
    #[strum(serialize = "，")]
    Comma,
    #[strum(serialize = "、")]
    DunComma,
    #[strum(serialize = "：")]
    Colon,
    #[strum(serialize = "！")]
    Exclamation,
    #[strum(serialize = "？")]
    Question,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn parsing_mantram_string_works() {
        let input =
            "na mo ciu khu ciu nan kuan she yin phu sat. pai chien wan yi fo.
南無救苦救難觀世音菩薩。百千萬億佛。

heng he sha shu fo. u liang kung te fo.
恒河沙數佛。無量功德佛。
";

        assert!(parse_mantram_string(input).is_ok());
    }

    #[rstest]
    #[case("。", Character::Punc(Punctuation::Period))]
    #[case("，", Character::Punc(Punctuation::Comma))]
    #[case("、", Character::Punc(Punctuation::DunComma))]
    #[case("：", Character::Punc(Punctuation::Colon))]
    #[case("！", Character::Punc(Punctuation::Exclamation))]
    #[case("？", Character::Punc(Punctuation::Question))]
    #[should_panic]
    #[case(".", Character::Punc(Punctuation::Period))]
    #[should_panic]
    #[case(",", Character::Punc(Punctuation::Period))]
    #[should_panic]
    #[case(":", Character::Punc(Punctuation::Period))]
    #[should_panic]
    #[case("!", Character::Punc(Punctuation::Period))]
    #[should_panic]
    #[case("?", Character::Punc(Punctuation::Period))]
    fn parsing_punc_works(#[case] input: &str, #[case] expected: Character) {
        let output = punc(input).unwrap();
        assert_eq!(expected, output.1);
    }

    #[rstest]
    #[case("（念自己的名字）")]
    #[case("（）")]
    fn parsing_context_works(#[case] input: &str) {
        let output = context(input).unwrap();
        assert_eq!(Character::Context(input.to_string()), output.1);
    }

    #[rstest]
    #[case("<br/>")]
    #[case("<br />")]
    #[case("<br>")]
    fn parsing_linebreak_works(#[case] input: &str) {
        let output = linebreak(input).unwrap();
        assert_eq!(Character::Linebreak, output.1);
    }
}
