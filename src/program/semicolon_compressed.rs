use crate::program::Program;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CompressedInstruction {
    Change(i32),
    Move(i32),
    Input,
    Output,
    LoopEnter,
    LoopExit,
}

pub struct CompressedProgram {
    instructions: Vec<CompressedInstruction>,
    jumps: HashMap<usize, usize>,
    //jumps: Vec<(usize, usize)>,
}

impl CompressedProgram {
    pub fn new(code: impl AsRef<str>) -> Self {
        let first_pass = code.as_ref().chars().flat_map(Self::first_pass).collect();
        let second_pass = Self::second_pass(first_pass);
        let jumps = Self::calculate_jumps(&second_pass);

        Self {
            instructions: second_pass,
            jumps,
        }
    }

    fn first_pass(c: char) -> Option<CompressedInstruction> {
        match c {
            '+' => Some(CompressedInstruction::Change(1)),
            '-' => Some(CompressedInstruction::Change(-1)),
            '<' => Some(CompressedInstruction::Move(-1)),
            '>' => Some(CompressedInstruction::Move(1)),
            ',' => Some(CompressedInstruction::Input),
            '.' => Some(CompressedInstruction::Output),
            '[' => Some(CompressedInstruction::LoopEnter),
            ']' => Some(CompressedInstruction::LoopExit),
            _ => None,
        }
    }

    fn second_pass(first_pass: Vec<CompressedInstruction>) -> Vec<CompressedInstruction> {
        let mut output: Vec<CompressedInstruction> = vec![];
        let mut active_instruction: Option<CompressedInstruction> = None;

        for instruction in first_pass.into_iter() {
            match (active_instruction, instruction) {
                (
                    Some(CompressedInstruction::Change(base)),
                    CompressedInstruction::Change(addend),
                ) => {
                    active_instruction =
                        Some(CompressedInstruction::Change(base.wrapping_add(addend)))
                }
                (Some(old), CompressedInstruction::Change(_)) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(CompressedInstruction::Move(base)), CompressedInstruction::Move(addend)) => {
                    active_instruction =
                        Some(CompressedInstruction::Move(base.wrapping_add(addend)))
                }
                (Some(old), CompressedInstruction::Move(_)) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(CompressedInstruction::Change(_)), CompressedInstruction::Input) => {
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstruction::Input) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstruction::Output) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstruction::LoopEnter) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstruction::LoopExit) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (None, instruction) => {
                    active_instruction = Some(instruction);
                }
            }
        }
        output.push(active_instruction.unwrap());

        output
    }

    fn calculate_jumps(second_pass: &[CompressedInstruction]) -> HashMap<usize, usize> {
        let mut output: HashMap<usize, usize> = HashMap::new();

        for (index, instruction) in second_pass.iter().enumerate() {
            match instruction {
                CompressedInstruction::LoopEnter => {
                    let value: usize = Self::find_matching_exit(second_pass, index);
                    output.insert(index, value);
                }
                CompressedInstruction::LoopExit => {
                    let value: usize = Self::find_matching_enter(second_pass, index);
                    output.insert(index, value);
                }
                _ => {}
            }
        }

        output
    }

    fn find_matching_exit(program: &[CompressedInstruction], mut index: usize) -> usize {
        let starting_index = index;
        let mut depth: usize = 1;
        while depth > 0 {
            index = unsafe { index.unchecked_add(1) };
            match program.get(index) {
                Some(CompressedInstruction::LoopEnter) => depth = unsafe { depth.unchecked_add(1) },
                Some(CompressedInstruction::LoopExit) => depth = unsafe { depth.unchecked_sub(1) },
                Some(_) => {}
                None => panic!("Unbalanced program at index {starting_index}"),
            }
        }

        index
    }

    fn find_matching_enter(program: &[CompressedInstruction], mut index: usize) -> usize {
        let starting_index = index;
        let mut depth: usize = 1;
        while depth > 0 {
            unsafe {
                index = index.unchecked_sub(1);
            }
            match program.get(index) {
                Some(CompressedInstruction::LoopEnter) => depth = unsafe { depth.unchecked_sub(1) },
                Some(CompressedInstruction::LoopExit) => depth = unsafe { depth.unchecked_add(1) },
                Some(_) => {}
                None => panic!("Unbalanced program at index {starting_index}"),
            }
        }

        index
    }

    pub fn get_jump_location(&self, index: usize) -> usize {
        *self
            .jumps
            .get(&index)
            // .iter()
            //.find_map(|(enter, exit)| if *enter == index { Some(exit) } else { None })
            .expect("Program attempted to jump to from invalid loop")
    }
}

impl Program<CompressedInstruction> for CompressedProgram {
    fn get<'a>(&self, index: usize) -> Option<CompressedInstruction> {
        self.instructions.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
