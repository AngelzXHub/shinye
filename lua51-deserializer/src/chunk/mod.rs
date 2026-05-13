use nom::IResult;

pub use header::Header;

use crate::{
    chunk::header::{Endianness, Format},
    function::Function,
    parse::ParseConfig,
};

pub mod header;

#[derive(Debug)]
pub struct Chunk<'a> {
    pub function: Function<'a>,
}

impl<'a> Chunk<'a> {
    pub fn parse(input: &'a [u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;
        assert_eq!(header.version_number, 0x51);
        assert_eq!(header.format, Format::Official);
        assert_eq!(header.endianness, Endianness::Little);
        let (_, parse_config) = ParseConfig {
            int_width: header.int_width,
            size_t_width: header.size_t_width,
            instr_width: header.instr_width,
            number_width: header.number_width,
            number_is_integral: header.number_is_integral,
        }
        .validate(input)?;
        let (input, function) = Function::parse(input, &parse_config)?;

        Ok((input, Self { function }))
    }
}
