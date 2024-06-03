#![allow(non_snake_case, clippy::empty_docs)]

use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::character::complete::{alpha1, anychar, line_ending};
use nom::combinator::{map, map_res, recognize, value};
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use serde::Serialize;
use tsify::Tsify;

/// Possible characters in a mantram string.
#[derive(Debug, Clone, PartialEq, Serialize, Tsify)]
#[serde(rename_all = "lowercase", tag = "type")]
#[tsify(into_wasm_abi)]
pub enum Character {
    Hanzi { char: char, sub: String },
    Punc { char: Punctuation },
    Context { content: String },
    Linebreak,
}

/// Parses the list of characters in a full mantram string.
///
/// A mantram string consists of one or more blocks of the subtitle + hanzi lines.
///
/// NOTE: the end of the string needs to have a newline (`\n`).
pub fn mantram_string(input: &str) -> IResult<&str, Vec<Character>> {
    let (input, chars) = separated_list0(tag("\n"), block_characters)(input)?;

    Ok((input, chars.into_iter().flatten().collect()))
}

/// Parses the characters in the block (group of subtitle + hanzi lines).
fn block_characters(input: &str) -> IResult<&str, Vec<Character>> {
    let (input, subs) = subtitle_line(input)?;

    hanzi_line_chars(input, subs)
}

fn subtitle_line(input: &str) -> IResult<&str, Vec<&str>> {
    many0(terminated(alpha1, take_while(|c: char| !c.is_alphabetic())))(input)
}

fn hanzi_line_chars<'a>(
    input: &'a str,
    subs: Vec<&'a str>,
) -> IResult<&'a str, Vec<Character>> {
    let mut subs = subs.iter();

    let (input, (chars, _)) = many_till(
        alt((
            punc,
            context,
            linebreak,
            map(anychar, |c: char| Character::Hanzi {
                char: c,
                sub: subs.next().unwrap().to_string(),
            }),
        )),
        line_ending,
    )(input)?;

    Ok((input, chars))
}

fn punc(input: &str) -> IResult<&str, Character> {
    let (input, p) = map_res(take(1usize), Punctuation::from_str)(input)?;

    Ok((input, Character::Punc { char: p }))
}

fn context(input: &str) -> IResult<&str, Character> {
    let (input, s) =
        recognize(delimited(tag("（"), take_until("）"), tag("）")))(input)?;

    Ok((
        input,
        Character::Context {
            content: s.to_string(),
        },
    ))
}

fn linebreak(input: &str) -> IResult<&str, Character> {
    value(
        Character::Linebreak,
        alt((tag("<br/>"), tag("<br />"), tag("<br>"))),
    )(input)
}

#[non_exhaustive]
#[derive(
    Debug,
    Clone,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
    Serialize,
    Tsify,
)]
#[tsify(into_wasm_abi)]
pub enum Punctuation {
    #[strum(to_string = "。")]
    #[serde(rename = "。")]
    Period,

    #[strum(to_string = "，")]
    #[serde(rename = "，")]
    Comma,

    #[strum(to_string = "、")]
    #[serde(rename = "、")]
    DunComma,

    #[strum(to_string = "：")]
    #[serde(rename = "：")]
    Colon,

    #[strum(to_string = "！")]
    #[serde(rename = "！")]
    Exclamation,

    #[strum(to_string = "？")]
    #[serde(rename = "？")]
    Question,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn parsing_single_block_mantram_string_works() {
        let input = "om, siu to li, siu to li, siu mo li, so pho ho.
唵，修多利，修多利，修摩利，娑婆訶。
";

        let expected = vec![
            Character::Hanzi {
                char: '唵',
                sub: "om".to_string(),
            },
            Character::Punc {
                char: Punctuation::Comma,
            },
            Character::Hanzi {
                char: '修',
                sub: "siu".to_string(),
            },
            Character::Hanzi {
                char: '多',
                sub: "to".to_string(),
            },
            Character::Hanzi {
                char: '利',
                sub: "li".to_string(),
            },
            Character::Punc {
                char: Punctuation::Comma,
            },
            Character::Hanzi {
                char: '修',
                sub: "siu".to_string(),
            },
            Character::Hanzi {
                char: '多',
                sub: "to".to_string(),
            },
            Character::Hanzi {
                char: '利',
                sub: "li".to_string(),
            },
            Character::Punc {
                char: Punctuation::Comma,
            },
            Character::Hanzi {
                char: '修',
                sub: "siu".to_string(),
            },
            Character::Hanzi {
                char: '摩',
                sub: "mo".to_string(),
            },
            Character::Hanzi {
                char: '利',
                sub: "li".to_string(),
            },
            Character::Punc {
                char: Punctuation::Comma,
            },
            Character::Hanzi {
                char: '娑',
                sub: "so".to_string(),
            },
            Character::Hanzi {
                char: '婆',
                sub: "pho".to_string(),
            },
            Character::Hanzi {
                char: '訶',
                sub: "ho".to_string(),
            },
            Character::Punc {
                char: Punctuation::Period,
            },
        ];

        let (input, chars) = mantram_string(input).unwrap();

        assert_eq!("", input);
        assert_eq!(expected, chars);
    }

    #[test]
    fn parsing_multi_block_mantram_string_works() {
        let input = "po jo po luo mi tuo sin cing.
般若波羅蜜多心經。<br/>

kuan ce cai phu sat.
觀自在菩薩。
";

        let expected = vec![
            Character::Hanzi {
                char: '般',
                sub: "po".to_string(),
            },
            Character::Hanzi {
                char: '若',
                sub: "jo".to_string(),
            },
            Character::Hanzi {
                char: '波',
                sub: "po".to_string(),
            },
            Character::Hanzi {
                char: '羅',
                sub: "luo".to_string(),
            },
            Character::Hanzi {
                char: '蜜',
                sub: "mi".to_string(),
            },
            Character::Hanzi {
                char: '多',
                sub: "tuo".to_string(),
            },
            Character::Hanzi {
                char: '心',
                sub: "sin".to_string(),
            },
            Character::Hanzi {
                char: '經',
                sub: "cing".to_string(),
            },
            Character::Punc {
                char: Punctuation::Period,
            },
            Character::Linebreak,
            Character::Hanzi {
                char: '觀',
                sub: "kuan".to_string(),
            },
            Character::Hanzi {
                char: '自',
                sub: "ce".to_string(),
            },
            Character::Hanzi {
                char: '在',
                sub: "cai".to_string(),
            },
            Character::Hanzi {
                char: '菩',
                sub: "phu".to_string(),
            },
            Character::Hanzi {
                char: '薩',
                sub: "sat".to_string(),
            },
            Character::Punc {
                char: Punctuation::Period,
            },
        ];

        let (input, chars) = mantram_string(input).unwrap();

        assert_eq!("", input);
        assert_eq!(expected, chars);
    }

    #[test]
    fn parsing_subtitle_line_works() {
        let input = "ciu hu shan sin ..., yi shen li khu nan.\n唵。";
        let (input, subs) = subtitle_line(input).unwrap();

        assert_eq!("唵。", input);
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
        let (input, chars) = hanzi_line_chars(input, subs).unwrap();

        assert_eq!("", input);
        assert_eq!(
            vec![
                Character::Hanzi {
                    char: '救',
                    sub: "ciu".to_string()
                },
                Character::Hanzi {
                    char: '護',
                    sub: "hu".to_string()
                },
                Character::Hanzi {
                    char: '善',
                    sub: "shan".to_string()
                },
                Character::Hanzi {
                    char: '信',
                    sub: "sin".to_string()
                },
                Character::Context {
                    content: "（念自己的名字）".to_string()
                },
                Character::Punc {
                    char: Punctuation::Comma
                },
                Character::Hanzi {
                    char: '一',
                    sub: "yi".to_string()
                },
                Character::Hanzi {
                    char: '身',
                    sub: "shen".to_string()
                },
                Character::Hanzi {
                    char: '離',
                    sub: "li".to_string()
                },
                Character::Hanzi {
                    char: '苦',
                    sub: "khu".to_string()
                },
                Character::Hanzi {
                    char: '難',
                    sub: "nan".to_string()
                },
                Character::Punc {
                    char: Punctuation::Period
                }
            ],
            chars
        );
    }

    #[rstest]
    #[case("。", Character::Punc { char: Punctuation::Period })]
    #[case("，", Character::Punc { char: Punctuation::Comma })]
    #[case("、", Character::Punc { char: Punctuation::DunComma })]
    #[case("：", Character::Punc { char: Punctuation::Colon })]
    #[case("！", Character::Punc { char: Punctuation::Exclamation })]
    #[case("？", Character::Punc { char: Punctuation::Question })]
    #[should_panic]
    #[case(".", Character::Punc { char: Punctuation::Period })]
    #[should_panic]
    #[case(",", Character::Punc { char: Punctuation::Period })]
    #[should_panic]
    #[case(":", Character::Punc { char: Punctuation::Period })]
    #[should_panic]
    #[case("!", Character::Punc { char: Punctuation::Period })]
    #[should_panic]
    #[case("?", Character::Punc { char: Punctuation::Period })]
    fn parsing_punc_works(#[case] input: &str, #[case] expected: Character) {
        let (i, char) = punc(input).unwrap();

        assert_eq!("", i);
        assert_eq!(expected, char);
    }

    #[rstest]
    #[case("（念自己的名字）")]
    #[case("（）")]
    fn parsing_context_works(#[case] input: &str) {
        let (i, char) = context(input).unwrap();

        assert_eq!("", i);
        assert_eq!(
            Character::Context {
                content: input.to_string()
            },
            char
        );
    }

    #[rstest]
    #[case("<br/>")]
    #[case("<br />")]
    #[case("<br>")]
    fn parsing_linebreak_works(#[case] input: &str) {
        let (i, char) = linebreak(input).unwrap();

        assert_eq!("", i);
        assert_eq!(Character::Linebreak, char);
    }
}
