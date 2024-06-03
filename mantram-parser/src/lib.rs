use std::str::FromStr;
use std::string::ToString;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::character::complete::{alpha1, line_ending, multispace0};
use nom::combinator::{map_res, recognize, value};
use nom::multi::{many_till, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use strum_macros::EnumString;
// use wasm_bindgen::prelude::*;

pub fn parse_mantram_string(input: &str) -> Result<Vec<Character>, String> {
    let (_, chars) = separated_list0(tag("\n"), block_characters)(input)
        .or(Err("parsing failed".to_string()))?;

    Ok(chars.into_iter().flatten().collect())
}

fn block_characters(input: &str) -> IResult<&str, Vec<Character>> {
    let (input, subs) = subtitle_line(input)?;

    hanzi_line_chars(input, subs)
}

fn subtitle_line(input: &str) -> IResult<&str, Vec<&str>> {
    fn non_alpha(i: &str) -> IResult<&str, &str> {
        take_while(|c: char| !c.is_alphabetic() && c != '\n')(i)
    }

    terminated(separated_list0(non_alpha, alpha1), multispace0)(input)
}

fn hanzi_line_chars<'a>(
    input: &'a str,
    subs: Vec<&'a str>,
) -> IResult<&'a str, Vec<Character>> {
    let (input, chars) = many_till(take(1usize), line_ending)(input)?;

    // TODO

    Ok((input, vec![]))
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
    Hanzi {
        char: &'static str,
        sub: &'static str,
    },
    Punc(Punctuation),
    Context(String),
    Linebreak,
}

#[derive(Debug, Clone, PartialEq, EnumString, strum_macros::Display)]
#[non_exhaustive]
pub enum Punctuation {
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
        let input = "po jo po luo mi tuo sin cing.
般若波羅蜜多心經。<br/>

kuan ce cai phu sat.
觀自在菩薩。
";

        let expected = vec![
            Character::Hanzi {
                char: "般",
                sub: "po",
            },
            Character::Hanzi {
                char: "若",
                sub: "jo",
            },
            Character::Hanzi {
                char: "波",
                sub: "po",
            },
            Character::Hanzi {
                char: "羅",
                sub: "luo",
            },
            Character::Hanzi {
                char: "蜜",
                sub: "mi",
            },
            Character::Hanzi {
                char: "多",
                sub: "tuo",
            },
            Character::Hanzi {
                char: "心",
                sub: "sin",
            },
            Character::Hanzi {
                char: "經",
                sub: "cing",
            },
            Character::Punc(Punctuation::Period),
            Character::Linebreak,
            Character::Hanzi {
                char: "觀",
                sub: "kuan",
            },
            Character::Hanzi {
                char: "自",
                sub: "ce",
            },
            Character::Hanzi {
                char: "在",
                sub: "cai",
            },
            Character::Hanzi {
                char: "菩",
                sub: "phu",
            },
            Character::Hanzi {
                char: "薩",
                sub: "sat",
            },
            Character::Punc(Punctuation::Period),
        ];

        let chars = parse_mantram_string(input);
        assert!(chars.is_ok());
        assert_eq!(expected, chars.unwrap());
    }

    #[test]
    fn parsing_subtitle_line_works() {
        let input = "ciu hu shan sin ..., yi shen li khu nan.\n";
        let (_, subs) = subtitle_line(input).unwrap();

        assert_eq!(
            vec!["ciu", "hu", "shan", "sin", "yi", "shen", "li", "khu", "nan"],
            subs
        );
    }

    #[test]
    fn parsing_hanzi_line_chars_works() {
        let input = "ciu hu shan sin ..., yi shen li khu nan.
救護善信（念自己的名字），一身離苦難。
";
        let (input, subs) = subtitle_line(input).unwrap();
        let (_, chars) = hanzi_line_chars(input, subs).unwrap();

        assert_eq!(
            vec![
                Character::Hanzi {
                    char: "救",
                    sub: "ciu"
                },
                Character::Hanzi {
                    char: "護",
                    sub: "hu"
                },
                Character::Hanzi {
                    char: "善",
                    sub: "shan"
                },
                Character::Hanzi {
                    char: "信",
                    sub: "sin"
                },
                Character::Context("（念自己的名字）".to_string()),
                Character::Punc(Punctuation::Comma),
                Character::Hanzi {
                    char: "一",
                    sub: "yi"
                },
                Character::Hanzi {
                    char: "身",
                    sub: "shen"
                },
                Character::Hanzi {
                    char: "離",
                    sub: "li"
                },
                Character::Hanzi {
                    char: "苦",
                    sub: "khu"
                },
                Character::Hanzi {
                    char: "難",
                    sub: "nan"
                },
                Character::Punc(Punctuation::Period)
            ],
            chars
        );
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
