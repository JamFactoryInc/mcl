use crate::grammar::commands::execute;

pub enum ExecuteInstr {
    SubCommands(execute::Cmds)
}