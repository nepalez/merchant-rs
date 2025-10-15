use zeroize::Zeroize;

/// Sealed trait for newtypes with input sanitization and validation.
pub(crate) trait Sanitized<'a> {
    type Input: Sized;

    fn sanitize(input: Self::Input) -> Self;
}

#[inline]
pub(crate) fn filter_characters(output: &mut String, input: &str, filter: &'static str) {
    for c in input.chars() {
        if !c.is_ascii_control() && !c.is_whitespace() && !filter.contains(c) {
            output.push(c);
        }
    }
    output.shrink_to_fit();
}

#[inline]
pub(crate) fn trim_whitespaces(output: &mut String, input: &str) {
    for c in input.trim().chars() {
        if !c.is_ascii_control() {
            output.push(c);
        }
    }
    output.shrink_to_fit();
}
