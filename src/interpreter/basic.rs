use crate::interpreter::Interpreter;
use crate::program::Program;
use crate::program::basic::BasicInstruction;
use std::num::NonZero;

pub(crate) struct BasicMemoryBank {
    memory: Vec<u8>,
    index: usize,
}

impl BasicMemoryBank {
    pub fn new(size: NonZero<usize>) -> Self {
        let size = size.get();
        Self {
            memory: vec![0; size],
            index: 0,
        }
    }

    pub fn increment(&mut self) {
        self.memory[self.index] = self.memory[self.index].wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.memory[self.index] = self.memory[self.index].wrapping_sub(1);
    }

    pub fn move_left(&mut self) {
        self.index = ((self.memory.len() + self.index).wrapping_sub(1)) % self.memory.len();
    }

    pub fn move_right(&mut self) {
        self.index = self.index.wrapping_add(1) % self.memory.len();
    }

    pub fn input(&mut self, value: char) {
        if value as u8 >= 127 {
            self.memory[self.index] = 0;
        } else {
            self.memory[self.index] = value as u8;
        }
    }

    pub fn output(&self) -> char {
        self.memory[self.index] as char
    }

    pub fn is_zero(&self) -> bool {
        self.memory[self.index] == 0
    }
}

pub struct BasicInterpreter {
    memory: BasicMemoryBank,
    index: usize,
}

impl BasicInterpreter {
    pub fn new(memory_size: NonZero<usize>) -> Self {
        Self {
            memory: BasicMemoryBank::new(memory_size),
            index: 0,
        }
    }

    fn execute_instruction(
        &mut self,
        instruction: &BasicInstruction,
        program: &dyn Program<BasicInstruction>,
        input: &mut String,
    ) {
        match instruction {
            BasicInstruction::Increment => self.memory.increment(),
            BasicInstruction::Decrement => self.memory.decrement(),
            BasicInstruction::MoveLeft => self.memory.move_left(),
            BasicInstruction::MoveRight => self.memory.move_right(),
            BasicInstruction::Input => {
                if input.is_empty() {
                    self.memory.input(0 as char);
                } else {
                    let mut iter = input.chars();
                    self.memory.input(iter.next().unwrap_or(0 as char));
                    *input = iter.collect();
                }
            }
            BasicInstruction::Output => {
                print!("{}", self.memory.output())
            }
            BasicInstruction::LoopEnter => {
                if self.memory.is_zero() {
                    self.skip_loop(program)
                }
            }
            BasicInstruction::LoopExit => {
                if !self.memory.is_zero() {
                    self.reverse_loop(program)
                }
            }
        }
    }

    fn skip_loop(&mut self, program: &dyn Program<BasicInstruction>) {
        let starting_index = self.index;
        let mut depth: usize = 1; // we already encountered a loop enter to get here
        while depth > 0 {
            self.index = unsafe { self.index.unchecked_add(1) };
            match program.get(self.index) {
                Some(BasicInstruction::LoopEnter) => depth = unsafe { depth.unchecked_add(1) },
                Some(BasicInstruction::LoopExit) => depth = unsafe { depth.unchecked_sub(1) },
                Some(_) => {}
                None => panic!("Unbalanced program at index {starting_index}"),
            }
        }
    }

    fn reverse_loop(&mut self, program: &dyn Program<BasicInstruction>) {
        let starting_index = self.index;
        let mut depth: usize = 1; // we already encountered a loop exit to get here
        while depth > 0 {
            unsafe {
                self.index = self.index.unchecked_sub(1);
            }
            if self.index == 0 {
                std::process::exit(1);
            }
            match program.get(self.index) {
                Some(BasicInstruction::LoopEnter) => depth = unsafe { depth.unchecked_sub(1) },
                Some(BasicInstruction::LoopExit) => depth = unsafe { depth.unchecked_add(1) },
                Some(_) => {}
                None => panic!("Unbalanced program at index {starting_index}"),
            }
        }
    }
}

impl Interpreter<BasicInstruction> for BasicInterpreter {
    fn execute_program(
        &mut self,
        program: &dyn Program<BasicInstruction>,
        input: impl ToString,
    ) -> usize {
        self.index = 0;
        let mut instruction_count: usize = 0;
        let mut input = input.to_string();

        while let Some(instruction) = program.get(self.index) {
            self.execute_instruction(&instruction, program, &mut input);
            self.index = self.index.wrapping_add(1);
            instruction_count += 1;
        }

        instruction_count
    }
}
