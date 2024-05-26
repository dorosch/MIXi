use crate::instruction::Instruction;

pub struct Program {
  pub instructions: Vec<Instruction>,
}

impl Program {
  pub fn new() -> Self {
    Self {
      instructions: Vec::new(),
    }
  }

  pub fn add(&mut self, instruction: Instruction) {
    self.instructions.push(instruction);
  }
}
