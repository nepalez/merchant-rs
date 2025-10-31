/// Represents payment method credentials that can be tokenized
pub enum Credentials<'a, Source: 'a + Sized> {
    Plain(Source),
    Tokenized(&'a str),
}
