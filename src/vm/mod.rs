use crate::grammar::entity::EntitySelector;
use crate::translate::bytecode::Instr;

pub type Function = usize;
pub enum Variable<'a> {
    Register(usize),
    Score(&'a EntitySelector),
    Nbt,
}

pub struct Layout<'a> {
    namespace: String,
    setup: Vec<Instr<'a>>,
    functions: Vec<McFunction<'a>>,
}

pub struct LayoutContext<'a> {
    layout: &'a mut Layout<'a>,
    instructions: Vec<Instr<'a>>
}

impl<'a> LayoutContext<'a> {
    pub fn add(&mut self, instruction: Instr) {
        let x = vec![1, 2, 3];
        let y = x.clone();
    }

    pub fn add_setup(&mut self, instruction: Instr<'a>) {
        self.layout.setup.push(instruction)
    }

}

pub struct McFunction<'a> {
    parent: usize,
    checksum: u128,
    name: String,
    instructions: Vec<Instr<'a>>
}