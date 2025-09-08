use crate::program::Program;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CompressedInstruction {
    Change(i32),
    Move(i32),
    Input,
    Output,
    LoopEnter(usize),
    LoopExit(usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CompressedInstructionStage1 {
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
        let third_pass = Self::third_pass(second_pass, &jumps);

        Self {
            instructions: third_pass,
            jumps,
        }
    }

    fn first_pass(c: char) -> Option<CompressedInstructionStage1> {
        match c {
            '+' => Some(CompressedInstructionStage1::Change(1)),
            '-' => Some(CompressedInstructionStage1::Change(-1)),
            '<' => Some(CompressedInstructionStage1::Move(-1)),
            '>' => Some(CompressedInstructionStage1::Move(1)),
            ',' => Some(CompressedInstructionStage1::Input),
            '.' => Some(CompressedInstructionStage1::Output),
            '[' => Some(CompressedInstructionStage1::LoopEnter),
            ']' => Some(CompressedInstructionStage1::LoopExit),
            _ => None,
        }
    }

    fn second_pass(
        first_pass: Vec<CompressedInstructionStage1>,
    ) -> Vec<CompressedInstructionStage1> {
        let mut output: Vec<CompressedInstructionStage1> = vec![];
        let mut active_instruction: Option<CompressedInstructionStage1> = None;

        for instruction in first_pass.into_iter() {
            match (active_instruction, instruction) {
                (
                    Some(CompressedInstructionStage1::Change(base)),
                    CompressedInstructionStage1::Change(addend),
                ) => {
                    active_instruction = Some(CompressedInstructionStage1::Change(
                        base.wrapping_add(addend),
                    ))
                }
                (Some(old), CompressedInstructionStage1::Change(_)) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (
                    Some(CompressedInstructionStage1::Move(base)),
                    CompressedInstructionStage1::Move(addend),
                ) => {
                    active_instruction =
                        Some(CompressedInstructionStage1::Move(base.wrapping_add(addend)))
                }
                (Some(old), CompressedInstructionStage1::Move(_)) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (
                    Some(CompressedInstructionStage1::Change(_)),
                    CompressedInstructionStage1::Input,
                ) => {
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstructionStage1::Input) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstructionStage1::Output) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstructionStage1::LoopEnter) => {
                    output.push(old);
                    active_instruction = Some(instruction);
                }
                (Some(old), CompressedInstructionStage1::LoopExit) => {
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

    fn calculate_jumps(second_pass: &[CompressedInstructionStage1]) -> HashMap<usize, usize> {
        let mut output: HashMap<usize, usize> = HashMap::new();

        for (index, instruction) in second_pass.iter().enumerate() {
            match instruction {
                CompressedInstructionStage1::LoopEnter => {
                    let value: usize = Self::find_matching_exit(second_pass, index);
                    output.insert(index, value);
                }
                CompressedInstructionStage1::LoopExit => {
                    let value: usize = Self::find_matching_enter(second_pass, index);
                    output.insert(index, value);
                }
                _ => {}
            }
        }

        output
    }

    fn find_matching_exit(program: &[CompressedInstructionStage1], mut index: usize) -> usize {
        let starting_index = index;
        let mut depth: usize = 1;
        while depth > 0 {
            index = unsafe { index.unchecked_add(1) };
            match program.get(index) {
                Some(CompressedInstructionStage1::LoopEnter) => {
                    depth = unsafe { depth.unchecked_add(1) }
                }
                Some(CompressedInstructionStage1::LoopExit) => {
                    depth = unsafe { depth.unchecked_sub(1) }
                }
                Some(_) => {}
                None => panic!("Unbalanced program at index {starting_index}"),
            }
        }

        index
    }

    fn find_matching_enter(program: &[CompressedInstructionStage1], mut index: usize) -> usize {
        let starting_index = index;
        let mut depth: usize = 1;
        while depth > 0 {
            unsafe {
                index = index.unchecked_sub(1);
            }
            match program.get(index) {
                Some(CompressedInstructionStage1::LoopEnter) => {
                    depth = unsafe { depth.unchecked_sub(1) }
                }
                Some(CompressedInstructionStage1::LoopExit) => {
                    depth = unsafe { depth.unchecked_add(1) }
                }
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

    fn third_pass(
        second_pass: Vec<CompressedInstructionStage1>,
        jumps: &HashMap<usize, usize>,
    ) -> Vec<CompressedInstruction> {
        second_pass
            .into_iter()
            .enumerate()
            .map(|(index, instruction)| match instruction {
                CompressedInstructionStage1::Change(v) => CompressedInstruction::Change(v),
                CompressedInstructionStage1::Move(v) => CompressedInstruction::Move(v),
                CompressedInstructionStage1::Input => CompressedInstruction::Input,
                CompressedInstructionStage1::Output => CompressedInstruction::Output,
                CompressedInstructionStage1::LoopEnter => CompressedInstruction::LoopEnter(
                    *jumps.get(&index).expect("Unbalanced Program!"),
                ),
                CompressedInstructionStage1::LoopExit => CompressedInstruction::LoopExit(
                    *jumps.get(&index).expect("Unbalanced Program!"),
                ),
            })
            .collect()
    }
}

impl Program<CompressedInstruction> for CompressedProgram {
    fn get<'a>(&self, index: usize) -> Option<CompressedInstruction> {
        if index >= self.instructions.len() {
            return None;
        }
        Some(self.instructions[index])
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
