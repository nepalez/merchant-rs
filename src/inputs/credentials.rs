/// Represents payment method credentials that can be tokenized
pub enum Credentials<'a, Plain: 'a + Sized> {
    Plain(Plain),
    Tokenized(&'a str),
}
