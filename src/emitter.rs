use crate::parser::Statement;

#[derive(Debug)]
pub struct Emitter {
    statements: Vec<Statement>,
}

impl Emitter {
    pub fn new(statements: Vec<Statement>) -> Emitter {
        Emitter {
            statements
        }
    }

    // analysis
    pub fn analyze(&mut self) {}

    // analyze key points:
    // - follow identifiers (check types; if static, save as a number)
    // - if/while statements check for easier solutions, e.g. while x == true OR x > 1
    // - cache computed results
    // READ the wiki page about optimizing compiler

    // optimization passes
    pub fn optimize(&mut self) {}

    // write optimized instructions to file
    pub fn emit(&mut self) {}

    // run file
    pub fn execute_code(&mut self) {}
}
