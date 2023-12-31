use crate::grammar::commands::gamemode::Gamemode;
use crate::grammar::nbt::Nbt;
use crate::grammar::types::{DecimalRange, McIdentifier, UDecimalRange};
use crate::parse::unicode::UnicodeToken;
use crate::parse::*;
use crate::src_in::Source;
use crate::r#mod::RawString;
use crate::vm::LayoutContext;
use derive_more::Display;
use std::string::{ParseError, ToString};

pub enum ScoreboardEntitySelector {
    FakePlayer(String),
    EntitySelector(EntitySelector),
}

pub struct EntitySelector {
    target: EntitySelectorTarget,
    args: Vec<EntitySelectorArg>,
}

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
    Name(RawString),
}

impl Parser for EntitySelectorTarget {
    type State = ();
    const ERR: fn() -> String =
        || "expected one of `@a | @e | @p | @r | @s` or a player name".to_string();

    fn get_error(src: &mut Source) -> ParseError {
        ParseError::from(
            src,
            "expected one of `@a | @e | @p | @r | @s` or a player name",
        )
    }

    fn get_suggestions(partial: &[u8]) -> Vec<Suggestion> {
        if partial.len() == 0 {
            vec![
                Suggestion::of("@a".to_string(), "@a".to_string(), 0),
                Suggestion::of("@e".to_string(), "@e".to_string(), 0),
                Suggestion::of("@p".to_string(), "@p".to_string(), 0),
                Suggestion::of("@r".to_string(), "@r".to_string(), 0),
                Suggestion::of("@s".to_string(), "@s".to_string(), 0),
            ]
        } else if partial.len() == 1 && partial[0] == b'@' {
            vec![
                Suggestion::of("@a".to_string(), "a".to_string(), 1),
                Suggestion::of("@e".to_string(), "e".to_string(), 1),
                Suggestion::of("@p".to_string(), "p".to_string(), 1),
                Suggestion::of("@r".to_string(), "r".to_string(), 1),
                Suggestion::of("@s".to_string(), "s".to_string(), 1),
            ]
        } else {
            Vec::new()
        }
    }

    fn parse(src: &mut Source) {
        match src.peek() {
            b'@' => {
                src.next();
                match src.next().unwrap() {
                    b'a' => Some(Self::AtAll),
                    b'e' => Some(Self::AtEntities),
                    b'p' => Some(Self::AtPlayer),
                    b'r' => Some(Self::AtRandom),
                    b's' => Some(Self::AtSelf),
                    _ => None,
                };
            }
            x @ _ => {
                if UnicodeToken::McIdent.matches(x) {
                    Some(EntitySelectorTarget::Name(Matchers::repeat(
                        |char| UnicodeToken::McIdent.matches(char),
                        src,
                    )))
                } else {
                    None
                };
            }
        }
        todo!()
    }
}

pub enum EntitySelectorArg {
    Advancements,
    Distance(UDecimalRange),
    Dx(DecimalRange),
    Dy(DecimalRange),
    Dz(DecimalRange),
    Gamemode(Gamemode),
    Level(i32),
    Limit(i32),
    Name(McIdentifier),
    Nbt(Nbt),
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
