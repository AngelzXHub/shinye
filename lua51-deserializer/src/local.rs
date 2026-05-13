use std::ops::Range;

use nom::{multi::count, IResult};

use crate::parse::ParseConfig;
use crate::value::parse_string;

#[derive(Debug)]
pub struct Local<'a> {
    pub name: &'a [u8],
    pub range: Range<u64>,
}

impl<'a> Local<'a> {
    pub fn parse_list(input: &'a [u8], parse_config: &ParseConfig) -> IResult<&'a [u8], Vec<Self>> {
        let (input, length) = parse_config.int_as_usize(input)?;

        count(|input| Self::parse(input, parse_config), length)(input)
    }

    fn parse(input: &'a [u8], parse_config: &ParseConfig) -> IResult<&'a [u8], Self> {
        let (input, name) = parse_string(input, parse_config)?;
        let (input, start) = parse_config.parse_int(input)?;
        let (input, end) = parse_config.parse_int(input)?;

        Ok((
            input,
            Self {
                name: &name[..name.len() - 1],
                range: (start..end),
            },
        ))
    }
}
