use nom::{
    bytes::complete::is_not,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    sequence::delimited,
    IResult,
};

pub fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}
