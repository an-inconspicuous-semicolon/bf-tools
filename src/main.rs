use crate::interpreter::Interpreter;
use crate::interpreter::basic::BasicInterpreter;
use crate::program::basic::BasicProgram;
use std::num::NonZeroUsize;

pub mod interpreter;
pub mod program;

fn main() {
    let code = include_str!("../mandelbrot.bf");
    // let code = ",[.,]";
    let program = BasicProgram::new(code);
    let mut interpreter = BasicInterpreter::new(NonZeroUsize::new(30000).unwrap());
    interpreter.execute_program(&program, "Hello, World");
}
