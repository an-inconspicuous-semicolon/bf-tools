use crate::interpreter::ProgramInterpreter;
use crate::interpreter::semicolon_compressed::CompressedInterpreter;
use crate::program::semicolon_compressed::CompressedProgram;
use std::num::NonZeroUsize;

pub mod interpreter;
pub mod program;

fn main() {
    let code = include_str!("../mandelbrot.bf");
    // let code = ",[.,]";
    let program = CompressedProgram::new(code);
    let mut interpreter = CompressedInterpreter::new(NonZeroUsize::new(30000).unwrap());

    let starting_instant = std::time::Instant::now();
    let instruction_count = interpreter.execute_program(&program, "Hello, World");
    let finishing_instant = std::time::Instant::now();
    let difference = finishing_instant - starting_instant;
    println!(
        "\nExecuted {instruction_count} instructions in {}s ({} per sec)",
        difference.as_secs_f64(),
        instruction_count as f64 / difference.as_secs_f64()
    );
}
