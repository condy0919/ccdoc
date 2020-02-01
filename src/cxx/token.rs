/// [gram.lex] The kinds of C++ tokens
pub enum Kind {
    Identifier,
    Keyword,
    Literal,
    Operator,
    Punctuator,
}

/// A C++ token
#[derive(Eq, PartialEq)]
pub struct Token {
    kind: Kind,
    rep: String,
}
