use nom::{multi::count, IResult};

use crate::parse::ParseConfig;

#[derive(Debug)]
pub struct Position {
    pub instruction: usize,
    pub source: u64,
}

impl Position {
    pub fn parse<'a>(
        input: &'a [u8],
        parse_config: &ParseConfig,
    ) -> IResult<&'a [u8], Vec<Self>> {
        let (input, positions_length) = parse_config.int_as_usize(input)?;
        let (input, source_positions) =
            count(|input| parse_config.parse_int(input), positions_length)(input)?;

        Ok((
            input,
            source_positions
                .iter()
                .enumerate()
                .map(|(instruction, &source)| Self {
                    instruction,
                    source,
                })
                .collect(),
        ))
    }
}
