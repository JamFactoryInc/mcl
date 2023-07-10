use translate::bytecode::Instr;

pub type Register = usize;

pub struct Layout {
    namespace: String,
    setup: Vec<Instr>,
    functions: Vec<McFunction>,
}

pub struct LayoutContext {
    layout: Box<Layout>,
    instructions: Vec<Instr>
}

impl LayoutContext {
    pub fn add(&mut self, instruction: Instr) {
        let x = vec![1, 2, 3];
        let y = x.clone();
    }

    pub fn add_setup(&mut self, instruction: Instr) {
        self.layout.setup.push(instruction)
    }

}

pub struct McFunction {
    parent: usize,
    checksum: u128,
    name: String,
    instructions: Vec<Instr>
}