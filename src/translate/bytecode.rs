use crate::grammar::commands::Command;
use crate::grammar::entity::{EntitySelectorArg, EntitySelectorTarget};
use crate::grammar::nbt::NbtPath;
use crate::grammar::scoreboard::ScoreOperator;
use crate::grammar::transform::{Coordinate, Rotation};
use crate::grammar::types::{Decimal, McIdentifier, Resource};
use crate::parse::{ParseError, Suggestion};
use crate::vm::{Function, Variable};

pub enum OperatorPrecedence {
    /// (...)
    Parenthetical,
    /// -a, --a
    Prefix,
    /// a++
    Postfix,
    /// a * b
    Multiplicative,
    /// a + b
    Additive,
    /// a < b
    Comparative,
    /// a == b
    Equal,
    /// a || b
    Logical,
}

pub enum Instr<'a> {
    Declare { dest: Variable<'a>, val: i32 },
    DeclareCopy { dest: Variable<'a>, src: Variable<'a> },
    Alias { alias: Variable<'a>, actual: Variable<'a> },
    /// scoreboard players set dest reg val
    Set { dest: Variable<'a>, val: i32 },
    Scale { dest: Variable<'a>, factor: Decimal },
    Operate { op: ScoreOperator, lhs_dest: Variable<'a>, rhs: Variable<'a> },
    /// scoreboard players operation dest reg = src reg
    Copy { dest: Variable<'a>, src: Variable<'a> },
    Move { dest: Variable<'a>, src: Variable<'a> },
    Ref { dest: NbtPath, src: Variable<'a> },
    Deref { dest: Variable<'a>, src: NbtPath },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    XOr { dest_rhs: Variable<'a>, lhs: Variable<'a> },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    Or { dest_rhs: Variable<'a>, lhs: Variable<'a> },
    /// scoreboard players operation dest_lhs reg *= rhs reg
    And { dest_rhs: Variable<'a>, lhs: Variable<'a> },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    NAnd { dest_rhs: Variable<'a>, lhs: Variable<'a> },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    Not { dest: Variable<'a>, src: Variable<'a> },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    XNor { dest: Variable<'a>, src: Variable<'a> },

    /// terminated with End
    Expression { precedence: OperatorPrecedence },
    BinaryExpr (),

    /// terminated with [End][Instr::End]
    Function (  ),
    Call ( Function ),
    /// terminated with End
    If {  },
    /// terminated with End
    Unless {  },
    /// terminated with End
    For {  },
    /// terminated with End
    While {  },
    /// terminated with End
    Match (Variable<'a>),
    End,

    Recurse,

    Resource (),

    Literal (&'static str),

    Coordinate (&'a Coordinate),
    Rotation (&'a Rotation),

    EntitySelector(EntitySelectorTarget),
    EntitySelectorArg(EntitySelectorArg),

    Command(Command),
    CommandArg(u32),


    ParseBreak { error: &'a ParseError, suggestion: &'a Suggestion }

}