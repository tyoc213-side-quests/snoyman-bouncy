#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
    WidthTooSmall(u32),
    HeightTooSmall(u32),
}

pub struct ParseArgs(std::env::Args);

impl ParseArgs {
    pub fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    pub fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            Some(x) => Ok(x),
            None => Err(ParseError::TooFewArgs)
        }
    }

    pub fn require_no_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok("".to_owned()),
        }
    }
}


pub fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s)),
        Ok(x) => Ok(x),
    }
}


pub fn parse_args() -> Result<(u32, u32), ParseError> {
    let mut args = ParseArgs::new();

    // skip the command name
    let _command_name = args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;
    args.require_no_arg()?;
    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    if width < 4 {
        return Err(ParseError::WidthTooSmall(width));
    }

    if height < 4 {
        return Err(ParseError::WidthTooSmall(height));
    }

    Ok((width, height))
}
