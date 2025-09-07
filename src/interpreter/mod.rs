pub mod basic;
pub mod semicolon_compressed;

use crate::program::Program;

pub trait Interpreter<T> {
    fn execute_program(&mut self, program: &dyn Program<T>, input: impl ToString) -> usize;
}

pub trait ProgramInterpreter<T> {
    fn execute_program(&mut self, program: &T, input: impl ToString) -> usize;
}
