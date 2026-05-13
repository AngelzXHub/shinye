use nom::{
    combinator::opt,
    multi::count,
    number::complete::le_u8,
    IResult,
};

use crate::{
    instruction::{position::Position, Instruction},
    local::Local,
    parse::ParseConfig,
    value::{self, Value},
};

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a [u8],
    pub line_defined: u64,
    pub last_line_defined: u64,
    pub number_of_upvalues: u8,
    pub vararg_flag: u8,
    pub maximum_stack_size: u8,
    pub code: Vec<Instruction>,
    pub constants: Vec<Value<'a>>,
    pub closures: Vec<Function<'a>>,
    pub positions: Vec<Position>,
    pub locals: Vec<Local<'a>>,
    pub upvalues: Vec<&'a [u8]>,
    pub number_of_parameters: u8,
}

impl<'a> Function<'a> {
    pub fn parse(input: &'a [u8], parse_config: &ParseConfig) -> IResult<&'a [u8], Self> {
        let (input, name) = value::parse_string(input, parse_config)?;
        let (input, line_defined) = parse_config.parse_int(input)?;
        let (input, last_line_defined) = parse_config.parse_int(input)?;
        let (input, number_of_upvalues) = le_u8(input)?;
        let (input, number_of_parameters) = le_u8(input)?;
        let (input, vararg_flag) = le_u8(input)?;
        let (input, maximum_stack_size) = le_u8(input)?;
        let (input, code_length) = parse_config.int_as_usize(input)?;
        let (input, code) = count(|input| Instruction::parse(input, parse_config), code_length)(input)?;
        let (input, constants_length) = parse_config.int_as_usize(input)?;
        let (input, constants) = count(|input| Value::parse(input, parse_config), constants_length)(input)?;
        let (input, closures_length) = parse_config.int_as_usize(input)?;
        let (input, closures) = count(|input| Self::parse(input, parse_config), closures_length)(input)?;
        let (input, positions) = opt(|input| Position::parse(input, parse_config))(input)?;
        let (input, locals) = opt(|input| Local::parse_list(input, parse_config))(input)?;
        let (input, upvalues) = opt(|input| value::parse_strings(input, parse_config))(input)?;

        Ok((
            input,
            Self {
                name,
                line_defined,
                last_line_defined,
                number_of_upvalues,
                vararg_flag,
                maximum_stack_size,
                code,
                constants,
                closures,
                positions: positions.unwrap_or_default(),
                locals: locals.unwrap_or_default(),
                upvalues: upvalues.unwrap_or_default(),
                number_of_parameters,
            },
        ))
    }
}
