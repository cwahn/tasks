// There are entries. The number of entry may be changed,,,?
// Restrictions + weightings => Make it dynamic. Vector of equation, weighing pair.
// Some are mandetory some are less. important => Weighted sum
// number of equation argument should be fixed. (No variadic function)
// However, each parameter may be vector or function with vector.
// In case of function, consider it as function composition.
// Restriction may be expressed in terms of intermediate value.
// Will evaluate if the system is valid or not in every iteration if anything changed.

// Try to push entry to transaction.
// Try to push transaction to ledger.

// Auto fill

// Projection

// The program is basically a REPL
// Read input
// Evaluate
// Print

// use std::{error::Error, io::stdin};

// type BoxErr = Box<dyn Error>;

// Read
// fn read_stdin() -> Result<Input, BoxErr> {
//     let mut buffer = String::new();
//     stdin().read_line(&mut buffer);
//     parse(buffer)
// }

// enum Output {}

// struct State {}

// struct Interpreter {
//     state: State,
// }

// impl Interpreter {
//     fn eval(&mut self, input: &Input) -> Output {}
// }

// fn effect(output: &Output) {}

fn main() {
    println!("Hello, world!");
}
