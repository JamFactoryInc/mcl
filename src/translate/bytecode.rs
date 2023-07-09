use crate::grammar::commands::Command;
use crate::grammar::entity::EntitySelectorTarget;
use crate::vm::Register;

pub enum Instr {
    /// scoreboard players set dest reg val
    Set { dest: Register, val: usize},
    /// scoreboard players operation arg0 reg < arg1 reg
    SetMin { dest_arg0: Register, arg1: Register },
    /// scoreboard players operation arg0 reg > arg1 reg
    SetMax { dest_arg0: Register, arg1: Register },
    /// scoreboard players operation arg0 reg >< arg1 reg
    Swap { lhs: Register, rhs: Register },
    Delete { register: Register },
    Scale { dest: Register },
    Add { dest_lhs: Register, src_rhs: Register },
    Sub { dest_lhs: Register, src_rhs: Register },
    SubRev { dest_rhs: Register, src_lhs: Register },
    Mul { dest_lhs: Register, src_rhs: Register },
    Div { dest_lhs: Register, src_rhs: Register },
    DivRev { dest_rhs: Register, src_lhs: Register },
    Mod { dest_lhs: Register, src_rhs: Register },
    ModRev { dest_rhs: Register, src_lhs: Register },
    /// scoreboard players operation dest reg = src reg
    Copy { dest: Register, src: Register },
    Ref { dest: Pat, src: Register },
    Deref { dest: Register, src: Path },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    XOr { dest_rhs: Register, lhs: Register },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    Or { dest_rhs: Register, lhs: Register },
    /// scoreboard players operation dest_lhs reg *= rhs reg
    And { dest_rhs: Register, lhs: Register },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    NAnd { dest_rhs: Register, lhs: Register },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    Not { dest: Register, src: Register },
    /// execute unless score rhs reg matches 0 store score lhs reg 1 if score lhs reg matches 0
    XNor { dest: Register, src: Register },

    Resource (&'static str),

    Literal (&'static str),

    EntitySelector(EntitySelectorTarget),
    EntitySelectorArg,

    Command(Command),
    CommandArg(u32),


}