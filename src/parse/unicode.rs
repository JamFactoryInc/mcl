pub enum UnicodeToken {
    /// Any non-whitespace, non-special Ascii character (33 - 126)
    Any,
    /// `[ \t]`
    WhiteSpace,
    /// `[\n\r]`
    NewLine,
    /// `[0-9]`
    Digit,
    /// `[0-9.]`
    Numeric,
    /// `[A-Fa-f0-9]`
    Hexadecimal,
    /// `[a-z]`
    Lower,
    /// `[A-Z]`
    Upper,
    /// `[A-z]`
    Alpha,
    /// `[A-z0-9]`
    AlphaNumeric,
    /// `[><=]`
    CompSymbol,
    /// `[/*+-%]`
    OpSymbol,
    /// `[=|&!^orandtx]`
    ///
    /// support for either literal 'and', 'or', 'xor', and 'not' (as well as derivative 'xnor', 'nand', etc.),
    /// but also typical c-style logical operators like `==`, `||`, `&&`, `!=`, `^`, etc.
    LogicSymbol,
    /// `[{}]`
    Brace,
    /// `[()]`
    Paren,
    Resource,
    McIdent,
    // same as `Any` but omits `@`
    ScoreVariableFirst,
    /// `[A-z_]`
    IdentFirst,
    /// `[A-z_0-9]`
    Ident,
    /// Matches anything other than the provided char
    Not (u8),
    /// Matches only the provided char
    Literal (u8),
}
pub trait AsciiUtils {
    #[inline(always)]
    fn between(self, lower: u8, upper: u8) -> bool {
        self >= lower && self <= upper
    }
}
impl AsciiUtils for u8 {}

impl UnicodeToken {
    pub fn matches(&self, char: u8) -> bool {
        match self {
            Self::Any => char.between(b'!', b'~'),
            Self::Digit => char.is_ascii_digit(),
            Self::Numeric => char.is_ascii_digit() || char == b'.',
            Self::Hexadecimal => char.is_ascii_hexdigit(),
            Self::Lower => char.is_ascii_lowercase(),
            Self::Upper => char.is_ascii_uppercase(),
            Self::Alpha => char.is_ascii_alphabetic(),
            Self::AlphaNumeric => char.is_ascii_alphanumeric(),
            Self::CompSymbol => char.between(b'<', b'>'),
            Self::OpSymbol => char.between(b'*', b'-') || char == b'%' || char == b'/',
            Self::LogicSymbol => char == b'!' || char == b'&' || char == b'|',
            Self::Brace => char == b'{' || char == b'}',
            Self::Paren => char == b'(' || char == b')',
            Self::Resource => char.between(b'-', b':') || char.is_alpha() || char == b'_',
            Self::McIdent => char.is_ascii_alphanumeric() || char == b'.' || char == b'+' || char == b'-' || char == b'_',
            Self::ScoreVariableFirst => char.between(b'A', b'~') || char.between(b'!', b'?'),
            Self::IdentFirst => char.is_ascii_alphabetic() || char == b'_',
            Self::Ident => char.is_ascii_alphanumeric() || char == b'_',
            Self::Not(x) => char != *x,
            Self::Literal(x) => char == *x,
            Self::WhiteSpace => char == b' ' || char == b'\t',
            Self::NewLine => char == b'\n' || char == b'\r',
        }
    }

    pub fn get_out_of_range_message(&self, char: u8) -> &str {
        match self {
            Self::Any => "expected non-whitespace ascii character",
            Self::Digit => "expected a number from 0-9",
            Self::Numeric => "expected one a number from 0-9 or '.'",
            Self::Hexadecimal => "expected one a number from 0-9 or a letter from a-f",
            Self::Lower => "expected a lower-case letter from a-z",
            Self::Upper => "expected an upper-case letter from A-Z",
            Self::Alpha => "expected a letter from A-z",
            Self::AlphaNumeric => "expected a letter from A-z or a number from 0-9",
            Self::CompSymbol => "expected a comparison symbol such as '<', '>', or '='",
            Self::OpSymbol => "expected an operator symbol such as '+', '-', '*', '/', or '%'",
            Self::LogicSymbol => char == b'!' || char == b'&' || char == b'|',
            Self::Brace => char == b'{' || char == b'}',
            Self::Paren => char == b'(' || char == b')',
            Self::Resource => char.between(b'-', b':') || char.is_alpha() || char == b'_',
            Self::McIdent => char.is_ascii_alphanumeric() || char == b'.' || char == b'+' || char == b'-' || char == b'_',
            Self::ScoreVariableFirst => char.between(b'A', b'~') || char.between(b'!', b'?'),
            Self::IdentFirst => char.is_ascii_alphabetic() || char == b'_',
            Self::Ident => char.is_ascii_alphanumeric() || char == b'_',
            Self::Not(x) => char != *x,
            Self::Literal(x) => char == *x,
            UnicodeToken::WhiteSpace => char == b' ' || char == b'\t',
            UnicodeToken::NewLine => char == b'\n' || char == b'\r',
        }
    }
}