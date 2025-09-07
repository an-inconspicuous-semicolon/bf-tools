use crate::program::Program;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BasicInstruction {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Input,
    Output,
    LoopEnter,
    LoopExit,
}

pub struct BasicProgram {
    instructions: Vec<BasicInstruction>,
}

impl BasicProgram {
    pub fn new(code: impl AsRef<str>) -> Self {
        let instructions = code
            .as_ref()
            .chars()
            .flat_map(Self::create_instruction)
            .collect();
        Self { instructions }
    }

    fn create_instruction(c: char) -> Option<BasicInstruction> {
        match c {
            '+' => Some(BasicInstruction::Increment),
            '-' => Some(BasicInstruction::Decrement),
            '<' => Some(BasicInstruction::MoveLeft),
            '>' => Some(BasicInstruction::MoveRight),
            ',' => Some(BasicInstruction::Input),
            '.' => Some(BasicInstruction::Output),
            '[' => Some(BasicInstruction::LoopEnter),
            ']' => Some(BasicInstruction::LoopExit),
            _ => None,
        }
    }
}

impl Program<BasicInstruction> for BasicProgram {
    fn get<'a>(&self, index: usize) -> Option<BasicInstruction> {
        self.instructions.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
