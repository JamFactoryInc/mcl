use std::io::repeat;
use derive_more::Display;
use crate::grammar::types::UDecimalRange;
use crate::parse::ParseError;
use crate::src_in::Source;
use crate::parse::*;
use crate::parse::unicode::UnicodeToken;
use crate::translate::bytecode::Instr;

#[derive(Display)]
pub enum EntitySelectorTarget {
    /// @a
    #[display(fmt = "@a")]
    AtAll,
    /// @e
    #[display(fmt = "@e")]
    AtEntities,
    /// @p
    #[display(fmt = "@p")]
    AtPlayer,
    /// @r
    #[display(fmt = "@r")]
    AtRandom,
    /// @s
    #[display(fmt = "@s")]
    AtSelf,
    #[display(fmt = "{}", "String::from_utf8(_0)")]
    Name (Vec<u8>),
}
impl Parser<EntitySelectorTarget> for EntitySelectorTarget {
    fn get_error(&self, src: &Source) -> ParseError {
        ParseError::from(src, "expected one of `@a | @e | @p | @r | @s` or a player name")

    }

    fn get_suggestions(&self, partial: &[u8]) -> Vec<Suggestion> {
        if partial.len() == 0 {
            vec![
                Suggestion::of("@a", "@a", 0),
                Suggestion::of("@e", "@e", 0),
                Suggestion::of("@p", "@p", 0),
                Suggestion::of("@r", "@r", 0),
                Suggestion::of("@s", "@s", 0),
            ]
        } else if partial.len() == 1 && partial[0] == b'@' {
            vec![
                Suggestion::of("@a", "a", 1),
                Suggestion::of("@e", "e", 1),
                Suggestion::of("@p", "p", 1),
                Suggestion::of("@r", "r", 1),
                Suggestion::of("@s", "s", 1),
            ]
        } else {
            Vec::new()
        }
    }

    fn parse(&self, src: &mut Source) -> Option<EntitySelectorTarget> {
        match src.peek() {
            b'@' => {
                src.next();
                match src.next().unwrap() {
                    b'a' => Some(Self::AtAll),
                    b'e' => Some(Self::AtEntities),
                    b'p' => Some(Self::AtPlayer),
                    b'r' => Some(Self::AtRandom),
                    b's' => Some(Self::AtSelf),
                }
            }
            x @ _ => {
                if UnicodeToken::McIdent.matches(x) {
                    Some(EntitySelectorTarget::Name(Parser::repeat(|char| UnicodeToken::McIdent.matches(char), src)))
                } else {
                    None
                }
            }
        }
    }
}

pub enum EntitySelectorArg {
    Advancements,
    Distance (UDecimalRange),
    Dx,
    Dy,
    Dz,
    Gamemode,
    Level,
    Limit,
    Name,
    Nbt,
    Predicate,
    Scores,
    Sort,
    Tag,
    Team,
    X,
    Y,
    Z,
    XRot,
    YRot,
    ZRot,
}