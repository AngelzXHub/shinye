use enum_as_inner::EnumAsInner;
use nom::{
    bytes::complete::take,
    error::{Error, ErrorKind, ParseError},
    multi::count,
    number::complete::le_u8,
    Err, IResult,
};

use crate::parse::ParseConfig;

#[derive(Debug, EnumAsInner)]
pub enum Value<'a> {
    Nil,
    Boolean(bool),
    Number(f64),
    String(&'a [u8]),
}

impl<'a> Value<'a> {
    pub fn parse(input: &'a [u8], parse_config: &ParseConfig) -> IResult<&'a [u8], Self> {
        let (input, kind) = le_u8(input)?;

        match kind {
            0 => Ok((input, Self::Nil)),
            1 => {
                let (input, value) = le_u8(input)?;

                Ok((input, Self::Boolean(value != 0)))
            }
            3 => {
                let (input, value) = parse_config.parse_number(input)?;

                Ok((input, Self::Number(value)))
            }
            4 => {
                let (input, value) = parse_string(input, parse_config)?;

                // TODO: lua bytecode actually allows the string to be completely empty
                // it sets the type to string but gc to NULL
                // this probably causes some weird behavior
                assert!(!value.is_empty());

                // exclude null terminator
                Ok((input, Self::String(&value[..value.len() - 1])))
            }
            _ => Err(Err::Failure(Error::from_error_kind(
                input,
                ErrorKind::Switch,
            ))),
        }
    }
}

pub fn parse_string<'a>(
    input: &'a [u8],
    parse_config: &ParseConfig,
) -> IResult<&'a [u8], &'a [u8]> {
    let (input, string_length) = parse_config.size_t_as_usize(input)?;
    take(string_length)(input)
}

pub fn parse_strings<'a>(
    input: &'a [u8],
    parse_config: &ParseConfig,
) -> IResult<&'a [u8], Vec<&'a [u8]>> {
    let (input, string_count) = parse_config.int_as_usize(input)?;
    let (input, strings) = count(|input| parse_string(input, parse_config), string_count)(input)?;

    Ok((input, strings))
}
