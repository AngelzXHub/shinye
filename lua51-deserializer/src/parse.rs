use nom::{
    error::{Error, ErrorKind, ParseError},
    number::complete::{le_f32, le_f64, le_i32, le_i64, le_u32, le_u64},
    Err, IResult,
};

#[derive(Debug, Clone, Copy)]
pub struct ParseConfig {
    pub int_width: u8,
    pub size_t_width: u8,
    pub instr_width: u8,
    pub number_width: u8,
    pub number_is_integral: bool,
}

impl ParseConfig {
    pub fn validate(self, input: &[u8]) -> IResult<&[u8], Self> {
        if !matches!(self.int_width, 4 | 8)
            || !matches!(self.size_t_width, 4 | 8)
            || !matches!(self.instr_width, 4 | 8)
            || !matches!(self.number_width, 4 | 8)
        {
            return Err(Err::Failure(Error::from_error_kind(input, ErrorKind::Verify)));
        }

        Ok((input, self))
    }

    pub fn parse_int(self, input: &[u8]) -> IResult<&[u8], u64> {
        match self.int_width {
            4 => le_u32(input).map(|(input, value)| (input, u64::from(value))),
            8 => le_u64(input),
            _ => Err(Err::Failure(Error::from_error_kind(input, ErrorKind::Verify))),
        }
    }

    pub fn parse_size_t(self, input: &[u8]) -> IResult<&[u8], u64> {
        match self.size_t_width {
            4 => le_u32(input).map(|(input, value)| (input, u64::from(value))),
            8 => le_u64(input),
            _ => Err(Err::Failure(Error::from_error_kind(input, ErrorKind::Verify))),
        }
    }

    pub fn parse_instruction(self, input: &[u8]) -> IResult<&[u8], u32> {
        let (input, value) = match self.instr_width {
            4 => le_u32(input).map(|(input, value)| (input, u64::from(value)))?,
            8 => le_u64(input)?,
            _ => return Err(Err::Failure(Error::from_error_kind(input, ErrorKind::Verify))),
        };

        let value = u32::try_from(value)
            .map_err(|_| Err::Failure(Error::from_error_kind(input, ErrorKind::MapRes)))?;

        Ok((input, value))
    }

    pub fn parse_number(self, input: &[u8]) -> IResult<&[u8], f64> {
        match (self.number_is_integral, self.number_width) {
            (false, 4) => le_f32(input).map(|(input, value)| (input, value as f64)),
            (false, 8) => le_f64(input),
            (true, 4) => le_i32(input).map(|(input, value)| (input, value as f64)),
            (true, 8) => le_i64(input).map(|(input, value)| (input, value as f64)),
            _ => Err(Err::Failure(Error::from_error_kind(input, ErrorKind::Verify))),
        }
    }

    pub fn int_as_usize(self, input: &[u8]) -> IResult<&[u8], usize> {
        let (input, value) = self.parse_int(input)?;
        let value = usize::try_from(value)
            .map_err(|_| Err::Failure(Error::from_error_kind(input, ErrorKind::MapRes)))?;

        Ok((input, value))
    }

    pub fn size_t_as_usize(self, input: &[u8]) -> IResult<&[u8], usize> {
        let (input, value) = self.parse_size_t(input)?;
        let value = usize::try_from(value)
            .map_err(|_| Err::Failure(Error::from_error_kind(input, ErrorKind::MapRes)))?;

        Ok((input, value))
    }
}
