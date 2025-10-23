use zeroize::Zeroize;

/// Sealed trait for newtypes with input sanitization and validation.
pub(crate) trait Sanitized {
    fn sanitize(input: &str) -> Self;
}

pub(crate) fn filter_characters(output: &mut String, input: &str, filter: &'static str) {
    for c in input.chars() {
        if !c.is_ascii_control() && !c.is_whitespace() && !filter.contains(c) {
            output.push(c);
        }
    }
    output.shrink_to_fit();
}

pub(crate) fn trim_whitespaces(output: &mut String, input: &str) {
    for c in input.trim().chars() {
        if !c.is_ascii_control() {
            output.push(c);
        }
    }
    output.shrink_to_fit();
}
