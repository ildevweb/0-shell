pub fn split_and_parsing_commands(input: &str) -> Vec<&str> {
    // Split the input by whitespace and return a vector of commands
    input.split_whitespace().collect()
}


pub enum ParseError {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    UnclosedBackslash,
    EmptyInput,
    Other(&'static str),
}

pub fn parse_args(current_input: &str) -> Result<Vec<&str>, ParseError> {
    let trimmed = current_input.trim();
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