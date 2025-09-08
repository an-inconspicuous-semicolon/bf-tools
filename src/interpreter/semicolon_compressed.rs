use crate::interpreter::ProgramInterpreter;
use crate::program::Program;
use crate::program::semicolon_compressed::{CompressedInstruction, CompressedProgram};
use std::num::NonZero;

pub(crate) struct CompressedMemoryBank {
    memory: Vec<i16>,
    index: usize,
}

impl CompressedMemoryBank {
    pub fn new(size: NonZero<usize>) -> Self {
        let size = size.get();
        Self {
            memory: vec![0; size],
            index: 0,
        }
    }

    pub fn change(&mut self, value: i32) {
        self.memory[self.index] = self.memory[self.index].wrapping_add(value as i16);
    }

    pub fn move_pointer(&mut self, value: i32) {
        if value.is_negative() {
            self.index = ((self.memory.len() + self.index)
                .wrapping_sub(value.unsigned_abs() as usize))
                % self.memory.len();
        } else {
            self.index = ((self.memory.len() + self.index)
                .wrapping_add(value.unsigned_abs() as usize))
                % self.memory.len();
        }
    }
    pub fn input(&mut self, value: char) {
        if value as u8 >= 127 {
            self.memory[self.index] = 0;
        } else {
            self.memory[self.index] = value as u8 as i16;
        }
    }

    pub fn output(&self) -> char {
        self.memory[self.index] as u8 as char
    }

    pub fn is_zero(&self) -> bool {
        self.memory[self.index] == 0
    }
}

pub struct CompressedInterpreter {
    memory: CompressedMemoryBank,
    index: usize,
}

impl CompressedInterpreter {
    pub fn new(memory_size: NonZero<usize>) -> Self {
        Self {
            memory: CompressedMemoryBank::new(memory_size),
            index: 0,
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: &CompressedInstruction,
        input: &mut String,
    ) -> usize {
        match instruction {
            CompressedInstruction::Change(v) => {
                self.memory.change(*v);
                v.unsigned_abs() as usize
            }
            CompressedInstruction::Move(v) => {
                self.memory.move_pointer(*v);
                v.unsigned_abs() as usize
            }
            CompressedInstruction::Input => {
                if input.is_empty() {
                    self.memory.input(0 as char);
                } else {
                    let mut iter = input.chars();
                    self.memory.input(iter.next().unwrap_or(0 as char));
                    *input = iter.collect();
                }
                1
            }
            CompressedInstruction::Output => {
                print!("{}", self.memory.output());
                1
            }
            CompressedInstruction::LoopEnter(v) => {
                if self.memory.is_zero() {
                    self.index = *v;
                    // self.skip_loop(program);
                    // using the jump table is slower for some reason, but I'll keep it here encase I find a way to make it faster
                    // self.index = program.get_jump_location(self.index);
                }
                1
            }
            CompressedInstruction::LoopExit(v) => {
                if !self.memory.is_zero() {
                    self.index = *v;
                    // self.reverse_loop(program);
                    // using the jump table is slower for some reason, but I'll keep it here encase I find a way to make it faster
                    // self.index = program.get_jump_location(self.index);
                }
                1
            }
        }
    }
}

impl ProgramInterpreter<CompressedProgram> for CompressedInterpreter {
    fn execute_program(&mut self, program: &CompressedProgram, input: impl ToString) -> usize {
        self.index = 0;
        let mut instruction_count: usize = 0;
        let mut input = input.to_string();

        while let Some(instruction) = program.get(self.index) {
            instruction_count += self.execute_instruction(&instruction, &mut input);
            self.index = self.index.wrapping_add(1);
        }

        instruction_count
    }
}
