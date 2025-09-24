

#[derive(Debug)]
pub enum ParseError {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    UnclosedBackslash,
    EmptyInput,
    Other(&'static str),
}

pub fn parse_args(current_input: &str) -> Result<Vec<&str>, ParseError> {
    let trimmed = current_input.trim();
    //println!("this is trimmed: {}", current_input);
    if trimmed.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    if trimmed.contains('\'') && !trimmed.matches('\'').count() % 2 == 0 {
        return Err(ParseError::UnclosedSingleQuote);
    }
    if trimmed.contains('\"') && !trimmed.matches('\"').count() % 2 == 0 {
        return Err(ParseError::UnclosedDoubleQuote);
    }
    if trimmed.ends_with('\\') {
        return Err(ParseError::UnclosedBackslash);
    }
    Ok(trimmed.split_whitespace().collect())
}