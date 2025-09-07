pub mod basic;

use crate::program::Program;

pub trait Interpreter<T> {
    fn execute_program(&mut self, program: &dyn Program<T>, input: impl ToString);
}
